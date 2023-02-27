use crate::{
    impl_unix_probe,
    process::{
        unix::{UnixProcess, UnixProcessCache},
        Pid,
    },
    ProcessServerError,
    ProcessServerResult,
};
use psutil::process::os::linux::ProcessExt;
use tracing::debug;

impl UnixProcess {
    pub fn from_psutil(pid: Pid) -> ProcessServerResult<Self> {
        debug!("Calling procfs for pid {}", pid);
        let process = psutil::process::Process::new(pid)?;

        let stat = process.procfs_stat()?;
        let status = process.procfs_status()?;
        let owner_name = users::get_user_by_uid(status.uid[0])
            .ok_or(ProcessServerError::UserNotFound(status.uid[0]))?
            .name()
            .to_owned();
        let mut args = process.cmdline_vec()?.unwrap_or(vec!["kernel".to_string()]);
        let _executable = args.remove(0);
        Ok(Self {
            pid,
            ppid: stat.ppid.unwrap_or(0) as Pid,
            name: stat.comm,
            owner_id: status.uid[0],
            owner_name,
            exe: process.exe()?,
            cwd: process.cwd()?,
            cmdline: args.join(" "),
            args,
        })
    }
}

#[derive(Default)]
pub struct PsutilProbe {
    pub(crate) cache: UnixProcessCache,
}

impl_unix_probe!(PsutilProbe, UnixProcess::from_psutil);


#[cfg(test)]
mod tests {
    use crate::process::unix::UnixProcess;
    use std::process::Command;
    use tracing::debug;

    #[test_log::test]
    fn test_process() {
        let mut command = Command::new("cat").arg("/etc/timezone").arg("&").spawn().expect("works");
        debug!("Running process {}", command.id());
        let process = UnixProcess::from_psutil(command.id()).expect("works");
        debug!("{:?}", process);
        command.kill().expect("works");
    }
}
