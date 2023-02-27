use crate::process::{Pid, ProcessProbe};
use std::{ffi::OsString, path::PathBuf};

pub(crate) const PROC_PATH: &str = "/proc";

#[derive(Debug, Clone)]
pub struct UnixProcess {
    pid:        Pid,
    ppid:       Pid,
    name:       String,
    owner_id:   u32,
    owner_name: OsString,
    exe:        PathBuf,
    cwd:        PathBuf,
    cmdline:    String,
    args:       Vec<String>,
}

impl StaticProcess for UnixProcess {
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
        self.owner_id.to_string()
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
mod procfs_probe;
mod psutil_probe;

use crate::process::traits::StaticProcess;
pub use cache::UnixProcessCache;
pub use manual_probe::ManualProbe as UnixManualProbe;
pub use procfs_probe::ProcfsProbe as UnixProcfsProbe;
pub use psutil_probe::PsutilProbe as UnixPsutilProbe;

pub type UnixProcessProbe = Box<dyn ProcessProbe<UnixProcess>>;

macro_rules! impl_unix_probe {
    ($probe:ty, $method:path) => {
        impl $crate::process::ProcessProbe<$crate::process::UnixProcess> for $probe {
            fn collect_processes(&mut self) -> std::io::Result<Vec<&$crate::process::UnixProcess>> {
                use $crate::cache::Cache;
                tracing::debug!("Called collect processes on unix probe");
                let entries = std::fs::read_dir($crate::process::PROC_PATH)?;
                // we can clear the cache here as we can ensure no early return will happen after
                self.cache.clear();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            // Note we don't check if it is a directory as this can also fail due to metadata permissions,
                            // we assume the layout of `/proc`
                            let pid = match entry.file_name().to_string_lossy().parse::<Pid>() {
                                Ok(pid) => pid,
                                Err(_) => {
                                    tracing::trace!("This entry is not a pid {:?}",entry.file_name());
                                    continue;
                                }
                            };
                            match $method(pid) {
                                Ok(process) => { self.cache.add(process); }
                                Err(err) => { tracing::debug!("Could not read process for pid {} with error {}",pid,err) }
                            }
                        }
                        Err(err) => {
                            // Note if we can not read it, it surely is not an error, this can be a lack of priviledge, a directory being removed
                            // while iterating and much more, on Unix there is no way to freeze (lock) correctly a filesystem while operating on it
                            tracing::debug!("Could not read this entry with error {}",err);
                        }
                    }
                }
                Ok(self.get_cached_processes())
            }

            fn get_cached_processes(&self) -> Vec<&$crate::process::UnixProcess> {
                use $crate::cache::Cache;
                self.cache.get()
            }

             fn obtain_channel(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<$crate::process::UnixProcess>{
                use $crate::cache::ChannelCache;
                self.cache.subscribe()
             }
        }

        impl $probe {
            pub fn boxed(self) -> $crate::process::UnixProcessProbe {
                Box::new(self)
            }
        }
    };
}

pub(crate) use impl_unix_probe;
