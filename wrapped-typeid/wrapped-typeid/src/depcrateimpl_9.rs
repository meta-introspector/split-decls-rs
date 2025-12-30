// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
# [cfg (not (no_const_type_id))] impl ConstTypeId { # [must_use] pub const fn of < T > () -> Self where T : ? Sized , { ConstTypeId { type_id_fn : typeid :: of :: < T > , } } # [inline] fn get (self) -> TypeId { (self . type_id_fn) () } }
};
}
