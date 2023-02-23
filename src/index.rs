use dashmap::{DashMap, DashSet};

pub type Index = DashMap<String, DashSet<String>>;
