pub trait Cache<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn add(&mut self, value: T) -> &T;
    fn clear(&mut self) -> Vec<T>;
    fn get(&self) -> Vec<&T>;
}

pub trait ChannelCache<T: Clone>: Cache<T> {
    fn subscribe(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<T>;
    fn send(&mut self, value: T);
}

pub trait AsCache<T> {
    type Cache: Cache<T>;
    fn cache(&self) -> &Self::Cache;
    fn cache_mut(&mut self) -> &mut Self::Cache;
}

pub trait AsChannelCache<T: Clone> {
    type Cache: ChannelCache<T>;
    fn cache(&self) -> &<Self as AsChannelCache<T>>::Cache;
    fn cache_mut(&mut self) -> &mut <Self as AsChannelCache<T>>::Cache;
}

// Note when negative impl are fully implemented we will be able to do this
// magic like that
//
// impl<T, {REPLACE_ME}> !AsCache<T> for {REPLACE_ME} {}
//
// impl<T, C> Cache<T> for C where C: AsCache<T> {
//     fn add(&mut self, value: T) -> &T {
//         self.cache().add(value)
//     }
//
//     fn clear(&mut self) -> Vec<T> {
//         self.cache().clear()
//     }
//
//     fn get(&self) -> Vec<&T> {
//         self.cache().get()
//     }
// }

// impl<T, {REPLACE_ME}> !AsChannelCache<T> for {REPLACE_ME} {}
//
// impl<T, C> ChannelCache<T> for C where C: AsChannelCache<T> {
//     fn subscribe(&mut self) -> tokio::sync::mpsc::UnboundedReceiver<T> {
//         self.cache_mut().subscribe()
//     }
//
//     fn send(&mut self, value: T) {
//         self.cache().send(value)
//     }
// }
