use crate::{manager::Manager, StaticProcess};
use std::{ffi::OsString, net::IpAddr, sync::Arc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, warn};
use warp::Reply;

const MESSAGE: &str = r#"
POST `/acquire_process_list`
GET `/processes`
GET `/search`
GET `/data`
"#;

pub trait Config: serde::de::DeserializeOwned + serde::Serialize + Default {}

#[async_trait::async_trait]
pub trait Server {
    type Config: Config;

    fn new(config: Self::Config) -> Self;

    async fn serve(self, manager: Arc<parking_lot::RwLock<Manager>>);
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[derive(derive_builder::Builder)]
pub struct WarpServerConfig {
    address: IpAddr,
    port:    u16,
}

impl Default for WarpServerConfig {
    fn default() -> Self {
        Self {
            address: [0, 0, 0, 0].into(),
            port:    3000,
        }
    }
}

impl Config for WarpServerConfig {
}

pub struct WarpServer {
    pub config: WarpServerConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JSONError {
    error: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JSONProcess {
    pid:      u32,
    name:     String,
    uid:      String,
    // Note we convert this to a lossy string because OSString can not be sent
    username: String,
}

impl JSONProcess {
    pub fn new<T: StaticProcess>(x: &T) -> Self {
        Self {
            pid: x.pid(),
            name: x.name(),
            uid: x.owner_id(),
            #[cfg(windows)]
            username: map_os_string(&x.owner_name()),
            #[cfg(unix)]
            username: map_os_string(&x.owner_name()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SearchParams {
    pid:      Option<u32>,
    username: Option<String>,
}


fn map_os_string(s: &OsString) -> String {
    match s.to_str() {
        None => {
            #[cfg(windows)]
            let buffer = std::os::windows::ffi::OsStrExt::encode_wide(s.as_os_str())
                .flat_map(|x| [(x >> 8) as u8, x as u8])
                .collect::<Vec<_>>();
            #[cfg(unix)]
            let buffer = std::os::unix::ffi::OsStrExt::as_bytes(s.as_os_str());
            base64::Engine::encode(&base64::prelude::BASE64_URL_SAFE, buffer)
        },
        Some(s) => s.to_string(),
    }
}


#[async_trait::async_trait]
impl Server for WarpServer {
    type Config = WarpServerConfig;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    async fn serve(mut self, manager: Arc<parking_lot::RwLock<Manager>>) {
        use futures_util::StreamExt;
        use warp::Filter;
        let manager1 = manager.clone();
        let acquire_route = warp::path("acquire_process_list").and(warp::post()).map(move || {
            debug!("Called acquire_process_list");
            let mut manager = manager1.write();
            match manager.process_probe_mut().collect_processes() {
                Ok(_) => warp::reply::Response::default(),
                Err(e) => {
                    warn!("Could not collect processes {:?}", e);
                    warp::reply::with_status(
                        warp::reply::json(&JSONError {
                            error: format!("Could not collect processes {:?}", e),
                        }),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                    .into_response()
                },
            }
        });
        let manager2 = manager.clone();
        let processes_route = warp::path("processes").and(warp::get()).map(move || {
            debug!("Called processes");
            let manager = manager2.read();
            let processes = manager
                .process_probe()
                .get_cached_processes()
                .iter()
                .map(|&x| JSONProcess::new(x))
                .collect::<Vec<_>>();
            warp::reply::json(&processes)
        });

        let manager3 = manager.clone();
        let search_route = warp::path("search")
            .and(warp::get())
            .and(warp::query::<SearchParams>())
            .map(move |params: SearchParams| {
                debug!("Called search");
                let manager = manager3.read();
                let processes = manager
                    .process_probe()
                    .get_cached_processes()
                    .iter()
                    .filter(|&p| {
                        #[cfg(windows)]
                        let username = map_os_string(&p.owner_name());
                        #[cfg(unix)]
                        let username = map_os_string(&p.owner_name());
                        params.username.as_ref().map(|x| username.eq(x)).unwrap_or(true)
                            && params.pid.as_ref().map(|x| p.pid().eq(x)).unwrap_or(true)
                    })
                    .map(|&x| JSONProcess::new(x))
                    .collect::<Vec<_>>();
                warp::reply::json(&processes)
            });
        let manager4 = manager.clone();
        let data_route = warp::path("data").and(warp::get()).map(move || {
            debug!("Called data");
            let mut manager = manager4.write();
            let rx = manager.process_probe_mut().obtain_channel();
            drop(manager);
            let rx = UnboundedReceiverStream::new(rx);
            let stream = rx.map(|p| warp::sse::Event::default().json_data(JSONProcess::new(&p)));
            warp::sse::reply(stream)
        });

        let default_route = warp::get().and(warp::path::end()).map(|| MESSAGE);

        let routes =
            default_route.or(acquire_route).or(processes_route).or(search_route).or(data_route);

        warp::serve(routes).run((self.config.address, self.config.port)).await
    }
}
