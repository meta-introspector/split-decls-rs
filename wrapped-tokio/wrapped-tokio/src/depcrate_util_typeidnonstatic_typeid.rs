// Generated macro for nonstatic_typeid (function)
macro_rules! Depcrate_util_typeidnonstatic_typeid {
() => {
// Module: crate::util::typeid
// Provides: {"nonstatic_typeid"}
// Dependencies: {}
# [inline (always)] fn nonstatic_typeid < T > () -> TypeId where T : ? Sized , { trait NonStaticAny { fn get_type_id (& self) -> TypeId where Self : 'static ; } impl < T : ? Sized > NonStaticAny for PhantomData < T > { # [inline (always)] fn get_type_id (& self) -> TypeId where Self : 'static , { TypeId :: of :: < T > () } } let phantom_data = PhantomData :: < T > ; NonStaticAny :: get_type_id (unsafe { mem :: transmute :: < & dyn NonStaticAny , & (dyn NonStaticAny + 'static) > (& phantom_data) }) }
};
}
