// Generated wrapped crate with macro-based items

include!("depcratetest_vec.rs");
include!("depcratetest_feature_vec.rs");
include!("modcrate.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
}
