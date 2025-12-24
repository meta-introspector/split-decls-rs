use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Drop for RecursiveGuard {
    fn drop(&mut self) {
        self.0.with(|is_empty| is_empty.store(true, Ordering::Relaxed));
    }
}
