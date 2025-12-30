// Generated macro for Safety (enum)
macro_rules! Depcrate_intrinsicSafety {
() => {
// Module: crate::intrinsic
// Provides: {"Safety"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub enum Safety { Safe , Unsafe (Vec < UnsafetyComment >) , }
};
}
