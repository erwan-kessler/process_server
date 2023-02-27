use libprocess_server::{Manager, ManagerConfig, Server, WarpServer, WarpServerConfig, WarpServerConfigBuilder};
use parking_lot::RwLock;
use std::{net::IpAddr, sync::Arc};
use std::str::FromStr;
use tracing_subscriber::util::SubscriberInitExt;
use clap::*;
#[cfg(unix)]
use libprocess_server::UnixProbe;
#[cfg(windows)]
use libprocess_server::WindowsProbe;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    address: String,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
    #[cfg(windows)]
    #[arg(value_enum, default_value_t = ProbeType::Sysinfo)]
    probe_type: ProbeType,
    #[cfg(unix)]
    #[arg(value_enum, default_value_t = ProbeType::Manual)]
    probe_type: ProbeType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProbeType {
    Manual,
    #[cfg(windows)]
    Sysinfo,
    #[cfg(unix)]
    Procfs,
    #[cfg(unix)]
    Psutil,
}

fn parse_config() -> (ManagerConfig, WarpServerConfig) {
    let cli = Cli::parse();
    let manager_config = match cli.probe_type {
        ProbeType::Manual => {
            #[cfg(windows)]
                let config =
                ManagerConfig {
                    typ: WindowsProbe::Manual
                };
            #[cfg(unix)]
                let config =
                ManagerConfig {
                    typ: UnixProbe::Manual
                };


            config
        }
        #[cfg(windows)]
        ProbeType::Sysinfo =>
            ManagerConfig {
                typ: WindowsProbe::Sysinfo
            },
        #[cfg(unix)]
        ProbeType::Procfs =>
            ManagerConfig {
                typ: UnixProbe::Procfs
            },
        #[cfg(unix)]
        ProbeType::Psutil =>
            ManagerConfig {
                typ: UnixProbe::Psutil
            },
    };
    let address = match IpAddr::from_str(cli.address.as_str()) {
        Ok(x) => { x }
        Err(e) => {
            tracing::warn!("Error while parsing address defaulting {e:?}");
            IpAddr::from([127, 0, 0, 1])
        }
    };
    let server_config = WarpServerConfigBuilder::default()
        .address(address)
        .port(cli.port)
        .build().unwrap_or(WarpServerConfig::default());

    (manager_config, server_config)
}

#[tokio::main]
async fn main() {
    let (manager_config, server_config) = parse_config();
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).finish().init();
    let manager = Manager::new(manager_config);
    let server = WarpServer::new(server_config);
    server.serve(Arc::new(RwLock::new(manager))).await
}
