use crate::{
    process::{impl_channel_process_cache, impl_process_cache},
    ChannelProcessCache,
    WindowsManualProbe,
    WindowsProcess,
    WindowsSysinfoProbe,
};

pub type WindowsProcessCache = ChannelProcessCache<WindowsProcess>;

impl_process_cache!(WindowsManualProbe, WindowsProcess, WindowsProcessCache);
impl_channel_process_cache!(WindowsManualProbe, WindowsProcess, WindowsProcessCache);

impl_process_cache!(WindowsSysinfoProbe, WindowsProcess, WindowsProcessCache);
impl_channel_process_cache!(WindowsSysinfoProbe, WindowsProcess, WindowsProcessCache);
