use crate::{cache::ChannelCache, process::Pid};

use std::{ffi::OsString, path::PathBuf};

/// A trait that unify probe on each OSes
///
/// Note: this was left intentionally very simple as this is not meant as a real
/// probe If we wanted we would have a function to provide a way to add/remove
/// new process from a kernel callback process probe
pub trait ProcessProbe<T: StaticProcess + Clone>: ChannelCache<T> + Sync + Send {
    /// Collect all processes that can be read with the current permissions
    fn collect_processes(&mut self) -> std::io::Result<Vec<&'_ T>>;
    /// Get the current processes acquired
    fn get_cached_processes(&self) -> Vec<&'_ T>;
    /// Get a stream of the current processes acquired
    fn obtain_channel(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<T>;
}

/// A trait that give a static view to a process
///
/// Note: this also means that any change to the process will not be reflected
/// later on
pub trait StaticProcess {
    /// The process id is a unique identifier that allow to refer to a "running"
    /// process
    ///
    /// Note: this can be reused
    fn pid(&self) -> Pid;
    /// The parent process id is a unique identifier that allow to refer to the
    /// parent of the "running" process
    ///
    /// Note:
    ///  * This can be spoofed on Windows
    ///  * On Unix, 0 means no parent (usually init)
    fn ppid(&self) -> Pid;
    /// The name is usually what we refer as argv[0] and usually is the path of
    /// the file executed
    ///
    /// Note: this can be modified when loading the executable (especially for
    /// in memory)
    fn name(&self) -> String;
    /// The owner identifier is the unique way to identify who own that process
    /// (technically it's the user under which the process was launched under)
    ///
    /// We chose to only represent RUID here on Unix
    ///
    /// Note: there is 3 uid attached to a process on Unix
    ///  - Real UID: the one of the user/process that created this process (can
    ///    be forged with capabilities)
    ///  - Effective UID: used to evaluate priviledge, can be changed to
    ///    anything if 0 (root) else only RUID or SUID
    ///  - Saved UID: if set-uid bit set then this is equal to the owner of the
    ///    executable else this is RUID
    ///
    /// On windows this is way easier as this is a SID which is unique and
    /// contains all the information
    ///
    /// Note: On windows a SID is divided as such `S-R-X-Y1-Y2-Yn-1-Yn` where:
    ///  - S: Indicates that the string is a SID
    ///  - R: Indicates the revision level
    ///  - X: Indicates the identifier authority value
    ///  - Y: Represents a series of subauthority values, where n is the number
    ///    of values
    ///    * Yn is the relative identifier in the domain which could be mapped
    ///      to a RUID on Unix if we assume a single domain
    fn owner_id(&self) -> String;
    /// The name of the owner associated to this UID
    ///
    /// Note: this might be cached and not be relevant later on
    fn owner_name(&self) -> OsString;
    /// The process cmdline
    fn cmdline(&self) -> String;
    /// The arguments passed to the executable
    fn args(&self) -> Vec<String>;
    /// The process executable as an absolute path
    ///
    /// Note: this might be empty
    fn exe(&self) -> PathBuf;
    /// The process current working directory
    ///
    /// Note: this might be empty
    fn cwd(&self) -> PathBuf;
}
