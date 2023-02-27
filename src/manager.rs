use serde::{Deserialize, Serialize};
use tracing::debug;

pub struct Manager {
    #[cfg(unix)]
    process_probe: crate::process::UnixProcessProbe,
    #[cfg(windows)]
    process_probe: crate::process::WindowsProcessProbe,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum UnixProbe {
    Procfs,
    #[default]
    Manual,
    Psutil,
}

#[cfg(unix)]
impl Into<crate::process::UnixProcessProbe> for UnixProbe {
    fn into(self) -> crate::process::UnixProcessProbe {
        match self {
            UnixProbe::Procfs => crate::process::UnixManualProbe::default().boxed(),
            UnixProbe::Manual => crate::process::UnixProcfsProbe::default().boxed(),
            UnixProbe::Psutil => crate::process::UnixPsutilProbe::default().boxed(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum WindowsProbe {
    Manual,
    #[default]
    Sysinfo,
}

#[cfg(windows)]
impl Into<crate::process::WindowsProcessProbe> for WindowsProbe {
    fn into(self) -> crate::process::WindowsProcessProbe {
        match self {
            WindowsProbe::Manual => crate::process::WindowsManualProbe::default().boxed(),
            WindowsProbe::Sysinfo => crate::process::WindowsSysinfoProbe::default().boxed(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ManagerConfig {
    #[cfg(unix)]
    pub typ: UnixProbe,
    #[cfg(windows)]
    pub typ: WindowsProbe,
}

impl Manager {
    pub fn new(config: ManagerConfig) -> Self {
        debug!("Running with config {:?}", &config);
        Self {
            #[cfg(unix)]
            process_probe:                 config.typ.into(),
            #[cfg(windows)]
            process_probe:                 config.typ.into(),
        }
    }

    #[cfg(unix)]
    pub fn process_probe_mut(&mut self) -> &mut crate::process::UnixProcessProbe {
        &mut self.process_probe
    }

    #[cfg(windows)]
    pub fn process_probe_mut(&mut self) -> &mut crate::process::WindowsProcessProbe {
        &mut self.process_probe
    }

    #[cfg(unix)]
    pub fn process_probe(&self) -> &crate::process::UnixProcessProbe {
        &self.process_probe
    }

    #[cfg(windows)]
    pub fn process_probe(&self) -> &crate::process::WindowsProcessProbe {
        &self.process_probe
    }
}
