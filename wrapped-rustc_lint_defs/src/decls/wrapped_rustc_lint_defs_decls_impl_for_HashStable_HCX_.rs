use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<HCX> HashStable<HCX> for LintId {
    #[inline]
    fn hash_stable(&self, hcx: &mut HCX, hasher: &mut StableHasher) {
        self.lint_name_raw().hash_stable(hcx, hasher);
    }
}
