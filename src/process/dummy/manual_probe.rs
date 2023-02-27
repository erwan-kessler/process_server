use crate::{
    process::{dummy::cache::DummyProcessCache, impl_dummy_probe},
    DummyProcess,
    Pid,
    ProcessServerResult,
};


impl DummyProcess {
    pub fn from_manual(pid: Pid) -> ProcessServerResult<Self> {
        Ok(Self { pid })
    }
}

#[derive(Default)]
pub struct ManualProbe {
    pub(crate) cache: DummyProcessCache,
}

impl_dummy_probe!(ManualProbe, DummyProcess::from_manual);

#[cfg(test)]
mod tests {}
