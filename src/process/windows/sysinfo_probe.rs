use crate::{
    process::impl_windows_probe,
    Pid,
    ProcessServerError,
    ProcessServerResult,
    WindowsProcess,
    WindowsProcessCache,
};
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, SystemExt, UserExt};

lazy_static::lazy_static! {
     pub(crate) static ref SYSTEM:std::sync::Arc<parking_lot::RwLock<sysinfo::System>>=std::sync::Arc::new(parking_lot::RwLock::new(sysinfo::System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()).with_users_list())));
}


impl WindowsProcess {
    pub fn from_sysinfo(pid: Pid) -> ProcessServerResult<Self> {
        let system = SYSTEM.read();
        // Note on windows DWORD is the max, I am sure Guillaume had its reason but
        // usize is not necessarily the size
        let process = system
            .processes()
            .get(&sysinfo::Pid::from_u32(pid))
            .ok_or(ProcessServerError::MissingPid(pid))?;
        Ok(Self {
            pid,
            ppid: process.parent().unwrap_or(sysinfo::Pid::from_u32(0)).as_u32(),
            name: process.name().to_string(),
            owner_id: process.user_id().map(|x| x.to_string()).unwrap_or(String::new()),
            owner_name: process
                .user_id()
                .and_then(|uid| {
                    system
                        .users()
                        .iter()
                        .find(|x| x.id() == uid)
                        .map(|user| user.name().to_string())
                })
                .unwrap_or(String::new())
                .into(),
            exe: process.exe().to_path_buf(),
            cwd: process.cwd().to_path_buf(),
            cmdline: process.cmd().join(" "),
            args: vec![],
        })
    }
}

#[derive(Default)]
pub struct SysinfoProbe {
    pub(crate) cache: WindowsProcessCache,
}

impl_windows_probe!(SysinfoProbe, WindowsProcess::from_sysinfo);

#[cfg(test)]
mod tests {}
