// Generated macro for of (function)
macro_rules! Depcrateof {
() => {
// Module: crate
// Provides: {"of"}
// Dependencies: {}
# [must_use] # [inline (always)] pub fn of < T > () -> TypeId where T : ? Sized , { trait NonStaticAny { fn get_type_id (& self) -> TypeId where Self : 'static ; } impl < T : ? Sized > NonStaticAny for PhantomData < T > { # [inline (always)] fn get_type_id (& self) -> TypeId where Self : 'static , { TypeId :: of :: < T > () } } let phantom_data = PhantomData :: < T > ; NonStaticAny :: get_type_id (unsafe { mem :: transmute :: < & dyn NonStaticAny , & (dyn NonStaticAny + 'static) > (& phantom_data) }) }
};
}
