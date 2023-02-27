pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(unix)]
    #[error("Error while calling procfs: {0}")]
    ProcFsError(#[from] procfs::ProcError),
    #[cfg(unix)]
    #[error("Error while calling psutil: {0}")]
    PsUtilError(#[from] psutil::process::ProcessError),
    #[error("Error while parsing Unix stat: {0}")]
    InvalidUnixStat(String),
    #[error("User not found: {0}")]
    UserNotFound(u32),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Missing pid: {0}")]
    MissingPid(u32),
    #[error("Not yet implemented for {0}")]
    Unimplemented(String),
}
