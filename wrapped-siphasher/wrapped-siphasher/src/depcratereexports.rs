// Generated macro for reexports (module)
macro_rules! Depcratereexports {
() => {
// Module: crate
// Provides: {"reexports"}
// Dependencies: {}
# [cfg (any (feature = "serde" , feature = "serde_std" , feature = "serde_no_std"))] pub mod reexports { pub use serde ; # [cfg (feature = "serde_json")] pub use serde_json ; }
};
}
