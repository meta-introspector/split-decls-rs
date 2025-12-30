// Generated macro for impl_315 (impl)
macro_rules! Depcrate_tableimpl_315 {
() => {
// Module: crate::table
// Provides: {"impl_315"}
// Dependencies: {}
# [cfg (feature = "display")] impl std :: fmt :: Display for Table { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let children = self . get_values () ; for (key_path , value) in children { crate :: encode :: encode_key_path_ref (& key_path , f , None , DEFAULT_KEY_DECOR) ? ; write ! (f , "=") ? ; crate :: encode :: encode_value (value , f , None , DEFAULT_VALUE_DECOR) ? ; writeln ! (f) ? ; } Ok (()) } }
};
}
