// Generated wrapped crate with macro-based items

include!("depcrate_foobar.rs");
include!("depcratefoo.rs");
include!("modcrate.rs");
include!("modcrate_foo_bar.rs");
include!("modcrate_foo.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_foo_bar!();
    Modcrate_foo!();
}
