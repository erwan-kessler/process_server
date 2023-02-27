use crate::{
    process::impl_windows_probe,
    Pid,
    ProcessServerError,
    ProcessServerResult,
    WindowsProcess,
    WindowsProcessCache,
};


impl WindowsProcess {
    pub fn from_manual(pid: Pid) -> ProcessServerResult<Self> {
        Err(ProcessServerError::Unimplemented(format!(
            "Windows manual for {pid}"
        )))
    }
}

#[derive(Default)]
pub struct ManualProbe {
    pub(crate) cache: WindowsProcessCache,
}

impl_windows_probe!(ManualProbe, WindowsProcess::from_manual);

#[cfg(test)]
mod tests {}
