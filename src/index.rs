use dashmap::{DashMap, DashSet};
use tracing::info;

pub struct Index {
    inner: DashMap<String, DashSet<String>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        info!("Dropping index...");

        // TODO: write the index to a file
        for e in self.inner.iter() {
            println!("{} -> {}", e.key(), e.value().len());

            let v = e.value();
            for e in v.iter() {
                info!("-> {}", e.key());
            }
        }
    }
}
