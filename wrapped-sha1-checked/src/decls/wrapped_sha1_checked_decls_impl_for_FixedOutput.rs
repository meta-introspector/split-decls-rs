use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FixedOutput for Sha1 {
    #[inline]
    fn finalize_into(mut self, out: &mut Output<Self>) {
        self.finalize_inner(out);
    }
}
