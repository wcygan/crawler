use dashmap::{DashMap, DashSet};
use tracing::info;

pub struct Index {
    pub inner: DashMap<String, DashSet<String>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }
}

impl Drop for Index {
    // TODO: write the index to a file
    fn drop(&mut self) {
        info!("Dropping index...");
        for e in self.inner.iter() {
            info!("{} -> {}", e.key(), e.value().len());
            for e in e.value().iter() {
                info!("\t-> {}", e.key());
            }
        }
    }
}
