mod channel_cache;
mod dummy;
mod process_cache;
mod traits;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub(crate) use channel_cache::impl_channel_process_cache;
pub use channel_cache::ChannelProcessCache;
pub(crate) use process_cache::impl_process_cache;
pub use process_cache::ProcessCache;

pub use dummy::*;
pub use traits::{ProcessProbe, StaticProcess};
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;

pub type Pid = u32;
