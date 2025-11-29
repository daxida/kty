pub mod cli;
pub mod download;
pub mod lang;
pub mod locale;
pub mod models;
pub mod tags;
pub mod utils;

use indexmap::{IndexMap, IndexSet};

pub type Map<K, V> = IndexMap<K, V>; // Preserve insertion order
pub type Set<K> = IndexSet<K>;
