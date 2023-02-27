use crate::{
    process::{impl_channel_process_cache, impl_process_cache},
    ChannelProcessCache,
    DummyManualProbe,
    DummyProcess,
};

pub type DummyProcessCache = ChannelProcessCache<DummyProcess>;

impl_process_cache!(DummyManualProbe, DummyProcess, DummyProcessCache);
impl_channel_process_cache!(DummyManualProbe, DummyProcess, DummyProcessCache);
