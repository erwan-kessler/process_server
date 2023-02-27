use crate::process::{Pid, ProcessProbe};
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Clone)]
pub struct DummyProcess {
    pid: Pid,
}

impl StaticProcess for DummyProcess {
    fn pid(&self) -> Pid {
        self.pid
    }

    fn ppid(&self) -> Pid {
        Default::default()
    }

    fn name(&self) -> String {
        Default::default()
    }

    fn owner_id(&self) -> String {
        Default::default()
    }

    fn owner_name(&self) -> OsString {
        Default::default()
    }

    fn cmdline(&self) -> String {
        Default::default()
    }

    fn args(&self) -> Vec<String> {
        Default::default()
    }

    fn exe(&self) -> PathBuf {
        Default::default()
    }

    fn cwd(&self) -> PathBuf {
        Default::default()
    }
}

mod cache;
mod manual_probe;

use crate::process::traits::StaticProcess;
pub use manual_probe::ManualProbe as DummyManualProbe;

pub type DummyProcessProbe = Box<dyn ProcessProbe<DummyProcess>>;

macro_rules! impl_dummy_probe {
    ($probe:ty, $method:path) => {
        impl $crate::process::ProcessProbe<$crate::process::DummyProcess> for $probe {
            fn collect_processes(&mut self) -> std::io::Result<Vec<&$crate::process::DummyProcess>> {
                use $crate::cache::Cache;
                self.cache.clear();
                self.cache.add($method(42).expect("Not failing"));
                Ok(self.get_cached_processes())
            }

            fn get_cached_processes(&self) -> Vec<&$crate::process::DummyProcess> {
                use $crate::cache::Cache;
                self.cache.get()
            }

            fn obtain_channel(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<$crate::process::DummyProcess>{
                use $crate::cache::ChannelCache;
                self.cache.subscribe()
            }
        }

        impl $probe {
            pub fn boxed(self) -> $crate::process::DummyProcessProbe {
                Box::new(self)
            }
        }
    };
}

pub(crate) use impl_dummy_probe;
