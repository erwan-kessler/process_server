use crate::{
    impl_unix_probe,
    process::{
        unix::{UnixProcess, UnixProcessCache},
        Pid,
    },
    ProcessServerError,
    ProcessServerResult,
};
use tracing::debug;

impl UnixProcess {
    pub fn from_procfs(pid: Pid) -> ProcessServerResult<Self> {
        debug!("Calling procfs for pid {}", pid);
        let process = procfs::process::Process::new(pid as i32)?;

        let stat = process.stat()?;
        let status = process.status()?;
        let owner_name = users::get_user_by_uid(status.ruid)
            .ok_or(ProcessServerError::UserNotFound(status.ruid))?
            .name()
            .to_owned();
        let mut args = process.cmdline()?;
        let _executable = args.remove(0);
        Ok(Self {
            pid,
            ppid: stat.ppid as Pid,
            name: stat.comm,
            owner_id: status.ruid,
            owner_name,
            exe: process.exe()?,
            cwd: process.cwd()?,
            cmdline: args.join(" "),
            args,
        })
    }
}

#[derive(Default)]
pub struct ProcfsProbe {
    pub(crate) cache: UnixProcessCache,
}

impl_unix_probe!(ProcfsProbe, UnixProcess::from_procfs);


#[cfg(test)]
mod tests {
    use crate::process::unix::UnixProcess;
    use std::process::Command;
    use tracing::debug;

    #[test_log::test]
    fn test_process() {
        let mut command = Command::new("cat").arg("/etc/timezone").arg("&").spawn().expect("works");
        debug!("Running process {}", command.id());
        let process = UnixProcess::from_procfs(command.id()).expect("works");
        debug!("{:?}", process);
        command.kill().expect("works");
    }
}
