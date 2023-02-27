use crate::process::{Pid, ProcessProbe};
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone)]
pub struct WindowsProcess {
    pid:        Pid,
    ppid:       Pid,
    name:       String,
    owner_id:   String,
    owner_name: OsString,
    exe:        PathBuf,
    cwd:        PathBuf,
    cmdline:    String,
    args:       Vec<String>,
}

impl StaticProcess for WindowsProcess {
    fn pid(&self) -> Pid {
        self.pid
    }

    fn ppid(&self) -> Pid {
        self.ppid
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn owner_id(&self) -> String {
        self.owner_id.clone()
    }

    fn owner_name(&self) -> OsString {
        self.owner_name.clone()
    }

    fn cmdline(&self) -> String {
        self.cmdline.clone()
    }

    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn exe(&self) -> PathBuf {
        self.exe.clone()
    }

    fn cwd(&self) -> PathBuf {
        self.cwd.clone()
    }
}

mod cache;
mod manual_probe;
mod sysinfo_probe;

use crate::process::traits::StaticProcess;
pub use cache::WindowsProcessCache;
pub use manual_probe::ManualProbe as WindowsManualProbe;
pub use sysinfo_probe::SysinfoProbe as WindowsSysinfoProbe;

pub type WindowsProcessProbe = Box<dyn ProcessProbe<WindowsProcess>>;

macro_rules! impl_windows_probe {
    ($probe:ty, $method:path) => {
        impl $crate::process::ProcessProbe<$crate::process::WindowsProcess> for $probe {
            fn collect_processes(&mut self) -> std::io::Result<Vec<&$crate::process::WindowsProcess>> {
                use $crate::cache::Cache;
                tracing::debug!("Called collect processes on windows probe");
                use sysinfo::SystemExt;
                use sysinfo::PidExt;
                $crate::process::windows::sysinfo_probe::SYSTEM.write().refresh_system();
                let read_lock=$crate::process::windows::sysinfo_probe::SYSTEM.read();
                let pids=read_lock.processes().keys().into_iter().map(|pid| pid.as_u32()).collect::<Vec<u32>>();
                drop(read_lock);
                for pid in pids{
                    match $method(pid) {
                        Ok(process) => { self.cache.add(process); }
                        Err(err) => { tracing::debug!("Could not read process for pid {} with error {}",pid,err) }
                    }
                }
                Ok(self.get_cached_processes())
            }

            fn get_cached_processes(&self) -> Vec<&$crate::process::WindowsProcess> {
                use $crate::cache::Cache;
                self.cache.get()
            }

            fn obtain_channel(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<$crate::process::WindowsProcess>{
                use $crate::cache::ChannelCache;
                self.cache.subscribe()
            }
        }

        impl $probe {
            pub fn boxed(self) -> $crate::process::WindowsProcessProbe {
                Box::new(self)
            }
        }
    };
}

pub(crate) use impl_windows_probe;
