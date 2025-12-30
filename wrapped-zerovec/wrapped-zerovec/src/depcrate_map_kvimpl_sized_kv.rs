// Generated macro for impl_sized_kv (macro)
macro_rules! Depcrate_map_kvimpl_sized_kv {
() => {
// Module: crate::map::kv
// Provides: {"impl_sized_kv"}
// Dependencies: {}
macro_rules ! impl_sized_kv { ($ ty : path) => { impl <'a > ZeroMapKV <'a > for $ ty { type Container = ZeroVec <'a , $ ty >; type Slice = ZeroSlice <$ ty >; type GetType = <$ ty as AsULE >:: ULE ; type OwnedType = $ ty ; } } ; }
};
}
