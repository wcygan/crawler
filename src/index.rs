use dashmap::{DashMap, DashSet};
use tracing::info;

pub struct Index {
    pub inner: DashMap<String, DashSet<String>>,
    file: Option<String>,
}

impl Index {
    pub fn new(file: Option<String>) -> Self {
        Self {
            inner: DashMap::new(),
            file,
        }
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        match self.file {
            Some(ref path) => {
                let mut file = std::fs::File::create(path).unwrap();
                match serde_json::to_writer(&mut file, &self.inner) {
                    Ok(_) => info!("Index written to {}", path),
                    Err(e) => info!("Failed to write index to {}: {}", path, e),
                }
            }
            None => {
                for e in self.inner.iter() {
                    info!("{} contained {} link(s)", e.key(), e.value().len());
                    for e in e.value().iter() {
                        info!("\t-> {}", e.key());
                    }
                }
            }
        }
    }
}
