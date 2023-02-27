use std::collections::HashMap;

use std::{ffi::OsStr, fs::File, io::Read, path::PathBuf};

use crate::{
    impl_unix_probe,
    process::{
        unix::{UnixProcess, UnixProcessCache},
        Pid,
        PROC_PATH,
    },
    ProcessServerError,
    ProcessServerResult,
};
use tracing::debug;


const STAT: &str = "stat";
const STATUS: &str = "status";
const CMD_LINE: &str = "cmdline";
const CWD: &str = "cwd";
const EXE: &str = "exe";

impl UnixProcess {
    pub fn from_manual(pid: Pid) -> ProcessServerResult<Self> {
        debug!("Calling manual for pid {}", pid);
        let process_path = PathBuf::from(PROC_PATH).join(pid.to_string());

        // See definition in https://man7.org/linux/man-pages/man5/proc.5.html at `/proc/[pid]/stat`
        let stat_path = process_path.join(STAT);
        let mut file = File::open(stat_path)?;
        let mut stat_content = String::new();
        let _ = file.read_to_string(&mut stat_content)?;

        // See definition in https://man7.org/linux/man-pages/man5/proc.5.html at `/proc/[pid]/status`
        let status_path = process_path.join(STATUS);
        let mut file = File::open(status_path)?;
        let mut status_content = String::new();
        let _ = file.read_to_string(&mut status_content)?;

        // See definition in https://man7.org/linux/man-pages/man5/proc.5.html at `/proc/[pid]/cmdline`
        let cmdline_path = process_path.join(CMD_LINE);
        let mut file = File::open(cmdline_path)?;
        let mut cmdline = String::new();
        let _ = file.read_to_string(&mut cmdline)?;
        let mut cmdline = cmdline.replace('\0', " ");
        if cmdline.ends_with(' ') {
            cmdline.pop();
        }

        // See definition in https://man7.org/linux/man-pages/man5/proc.5.html at `/proc/[pid]/cwd`
        // Note as this is a symlink, you might not have permissions, in that case we
        // return it as empty  (this is a deliberate choice for static process)
        let cwd_path = process_path.join(CWD);
        let cwd = match cwd_path.read_link() {
            Ok(e) => e,
            Err(e) => {
                debug!("Could not obtain cwd {e}");
                PathBuf::new()
            },
        };

        // See definition in https://man7.org/linux/man-pages/man5/proc.5.html at `/proc/[pid]/exe`
        // Note as this is a symlink, you might not have permissions, in that case we
        // return it as empty (this is a deliberate choice for static process)
        // Note: if the string (deleted) appear as a suffix, it is removed to ensure a
        // full valid path Note: we suppose linux 2.2 and later (supporting
        // earlier makes no sense in Rust, since 1.63.0- => Linux 2.6.32+)
        let exe_path = process_path.join(EXE);
        let exe = match exe_path.read_link() {
            Ok(mut e) => {
                const DELETED_PATTERN: &str = " (deleted)";
                if e.ends_with(DELETED_PATTERN) {
                    use std::os::unix::ffi::OsStrExt;
                    let r = e.as_os_str().as_bytes();
                    let r = &r[..r.len() - DELETED_PATTERN.len()];
                    e = PathBuf::from(OsStr::from_bytes(r));
                }
                e
            },
            Err(e) => {
                debug!("Could not obtain exe {e}");
                PathBuf::new()
            },
        };

        let Some(idx) = stat_content.find('(') else {
            return Err(ProcessServerError::InvalidUnixStat("Could not find comm start".to_string()));
        };
        let com = &stat_content[idx + 1..];
        let Some(idx) = com.rfind(')') else {
            return Err(ProcessServerError::InvalidUnixStat("Could not find comm end".to_string()));
        };
        let name = com[..idx].to_string();
        // we ignore the start with `PID (TASK_COMM) ` (notice the space after parens)
        let slice = &com[idx + 2..];

        let fields = slice.split(' ').collect::<Vec<&str>>();
        // fields[0] is state
        let ppid = fields[1].parse().map_err(|x| {
            ProcessServerError::InvalidUnixStat(format!("The ppid field is not u32 size: {}", x))
        })?;

        // Note: as defined, this is supposed to be a human readable file, so some
        // spaces needs to be removed sadly this is the only easy place to get
        // the UID informations Note: we shouldn't error if one of the non
        // interesting field has a wrong format but for simplification we do
        let status = status_content
            .lines()
            .map(|line| line.split_once(':').map(|(key, value)| (key, value.trim_start())))
            .collect::<Option<HashMap<_, _>>>()
            .ok_or(ProcessServerError::InvalidUnixStat(
                "/proc/[PID]/status had an invalid syntax".to_string(),
            ))?;
        let uids = status.get("Uid").ok_or(ProcessServerError::InvalidUnixStat(
            "Missing uid map in /proc/[PID]/status".to_string(),
        ))?;
        let (real, _effective_saved_fs) = uids.split_once(|x: char| x.is_whitespace()).ok_or(
            ProcessServerError::InvalidUnixStat(
                "Invalid uid mapping, not whitespace separated".to_string(),
            ),
        )?;
        let owner_id = real.parse().map_err(|e| {
            ProcessServerError::InvalidUnixStat(format!(
                "Invalid real uid, could not be casted to u32 size {e}"
            ))
        })?;
        let owner_name = users::get_user_by_uid(owner_id)
            .ok_or(ProcessServerError::UserNotFound(owner_id))?
            .name()
            .to_owned();

        let mut args = cmdline.split(' ').map(String::from).collect::<Vec<_>>();
        // remove the last element of the split (empty because they are lines)
        let _last = args.pop();
        // remove the executable since it doesn't matter (it will always be there)
        let _executable = args.remove(0);

        Ok(Self {
            pid,
            ppid,
            name,
            owner_id,
            owner_name,
            exe,
            cwd,
            cmdline,
            args,
        })
    }
}

#[derive(Default)]
pub struct ManualProbe {
    pub(crate) cache: UnixProcessCache,
}

impl_unix_probe!(ManualProbe, UnixProcess::from_manual);


#[cfg(test)]
mod tests {
    use crate::process::unix::UnixProcess;
    use std::process::Command;
    use tracing::debug;

    #[test_log::test]
    fn test_process() {
        let mut command = Command::new("cat").arg("/etc/timezone").arg("&").spawn().expect("works");
        debug!("Running process {}", command.id());
        let process = UnixProcess::from_manual(command.id()).expect("works");
        debug!("{:?}", process);
        command.kill().expect("works");
    }
}
