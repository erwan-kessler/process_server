use crate::{
    process::{
        impl_channel_process_cache,
        impl_process_cache,
        unix::UnixProcess,
        UnixManualProbe,
        UnixProcfsProbe,
    },
    ChannelProcessCache,
    UnixPsutilProbe,
};

pub type UnixProcessCache = ChannelProcessCache<UnixProcess>;

impl_process_cache!(UnixManualProbe, UnixProcess, UnixProcessCache);
impl_channel_process_cache!(UnixManualProbe, UnixProcess, UnixProcessCache);

impl_process_cache!(UnixProcfsProbe, UnixProcess, UnixProcessCache);
impl_channel_process_cache!(UnixProcfsProbe, UnixProcess, UnixProcessCache);

impl_process_cache!(UnixPsutilProbe, UnixProcess, UnixProcessCache);
impl_channel_process_cache!(UnixPsutilProbe, UnixProcess, UnixProcessCache);
