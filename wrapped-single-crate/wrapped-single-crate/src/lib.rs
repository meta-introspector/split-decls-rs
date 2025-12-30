// Generated wrapped crate with macro-based items

include!("depcrateother_1.rs");
include!("depcrate_submodule_codetest.rs");
include!("depcrate_submodulecode.rs");
include!("depcratesubmodule.rs");
include!("modcrate.rs");
include!("modcrate_submodule_code.rs");
include!("modcrate_submodule.rs");

// Execute all items
pub fn execute_all() {
    Modcrate!();
    Modcrate_submodule_code!();
    Modcrate_submodule!();
}
