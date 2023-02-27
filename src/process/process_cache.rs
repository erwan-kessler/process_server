use crate::{Cache, StaticProcess};

pub struct ProcessCache<T: StaticProcess> {
    inner: Vec<T>,
}

impl<T: StaticProcess> Default for ProcessCache<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: StaticProcess> Cache<T> for ProcessCache<T> {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn add(&mut self, value: T) -> &'_ T {
        tracing::debug!("Adding a new process {}", value.pid());
        self.inner.push(value);
        // SAFETY: an element was inserted above, else this would have panicked on
        // allocation
        self.inner.last().unwrap()
    }

    fn clear(&mut self) -> Vec<T> {
        tracing::debug!("Clearing cache");
        self.inner.drain(0..).collect::<Vec<_>>()
    }

    fn get(&self) -> Vec<&'_ T> {
        tracing::debug!("Obtaining a copy of the cache");
        self.inner.iter().collect()
    }
}


macro_rules! impl_process_cache {
    ($probe:ty, $cached:ty, $cache:ty) => {
        impl $crate::cache::AsCache<$cached> for $probe {
            type Cache = $cache;

            fn cache(&self) -> &Self::Cache {
                &self.cache
            }

            fn cache_mut(&mut self) -> &mut Self::Cache {
                &mut self.cache
            }
        }

        impl $crate::cache::Cache<$cached> for $probe{
            fn new() -> Self where Self:Sized{
                 Self::default()
            }

            fn add(&mut self, value: $cached) -> &$cached{
                $crate::cache::AsCache::<$cached>::cache_mut(self).add(value)
            }

            fn clear(&mut self) -> Vec<$cached>{
                $crate::cache::AsCache::<$cached>::cache_mut(self).clear()
            }

            fn get(&self) -> Vec<&$cached>{
                $crate::cache::AsCache::<$cached>::cache(self).get()
            }
        }
    };
}

pub(crate) use impl_process_cache;
