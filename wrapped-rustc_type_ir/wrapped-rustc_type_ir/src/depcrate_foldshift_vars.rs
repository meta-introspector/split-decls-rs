// Generated macro for shift_vars (function)
macro_rules! Depcrate_foldshift_vars {
() => {
// Module: crate::fold
// Provides: {"shift_vars"}
// Dependencies: {}
# [instrument (level = "trace" , skip (cx) , ret)] pub fn shift_vars < I : Interner , T > (cx : I , value : T , amount : u32) -> T where T : TypeFoldable < I > , { if amount == 0 || ! value . has_escaping_bound_vars () { value } else { value . fold_with (& mut Shifter :: new (cx , amount)) } }
};
}
