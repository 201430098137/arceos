mod hash_map;
pub use hash_map::HashMap;

#[cfg(feature = "alloc")]
extern crate alloc;
use alloc::collections::*;

