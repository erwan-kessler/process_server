use crate::{Cache, ChannelCache, Pid, ProcessCache, StaticProcess};
use std::collections::HashSet;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct ChannelProcessCache<T: StaticProcess> {
    channels: Vec<UnboundedSender<T>>,
    seen:     HashSet<(Pid, Pid)>,
    cache:    ProcessCache<T>,
}

impl<T: StaticProcess> Default for ChannelProcessCache<T> {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
            seen:     Default::default(),
            cache:    ProcessCache::new(),
        }
    }
}

impl<T: StaticProcess + Clone> Cache<T> for ChannelProcessCache<T> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn add(&mut self, value: T) -> &T {
        // this is not pid rollover safe at all,
        // we implement a basic check using 2 elements making it very unlikely that this
        // could collide
        // Note we never clear that list ever so this will grow in memory ;)
        if !self.seen.contains(&(value.pid(), value.ppid())) {
            self.send(value.clone());
        }
        self.seen.insert((value.pid(), value.ppid()));
        self.cache.add(value)
    }

    fn clear(&mut self) -> Vec<T> {
        self.cache.clear()
    }

    fn get(&self) -> Vec<&T> {
        self.cache.get()
    }
}

impl<T: StaticProcess + Clone> ChannelCache<T> for ChannelProcessCache<T> {
    fn subscribe(&mut self) -> UnboundedReceiver<T> {
        tracing::debug!(
            "Adding another subscriber to the existing {} subscribers",
            self.channels.len()
        );
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        self.channels.push(tx);
        rx
    }

    fn send(&mut self, value: T) {
        tracing::debug!("Sending a value to {} channels", self.channels.len());
        self.channels.retain(|x| x.send(value.clone()).is_ok());
    }
}


macro_rules! impl_channel_process_cache {
    ($probe:ty, $cached:ty, $cache:ty) => {
        impl $crate::cache::AsChannelCache<$cached> for $probe {
            type Cache = $cache;

            fn cache(&self) -> &Self::Cache {
                &self.cache
            }

            fn cache_mut(&mut self) -> &mut Self::Cache {
                &mut self.cache
            }
        }
        impl $crate::cache::ChannelCache<$cached> for $probe{
            fn subscribe(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<$cached>{
                $crate::cache::AsChannelCache::<$cached>::cache_mut(self).subscribe()
            }

            fn send(&mut self, value: $cached){
                $crate::cache::AsChannelCache::<$cached>::cache_mut(self).send(value)
            }
        }
    };
}

pub(crate) use impl_channel_process_cache;
