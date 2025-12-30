// Generated macro for impl_101 (impl)
macro_rules! Depcrate_errorimpl_101 {
() => {
// Module: crate::error
// Provides: {"impl_101"}
// Dependencies: {}
impl < I : Stream , C > ParserError < I > for ContextError < C > { type Inner = Self ; # [inline] fn from_input (_input : & I) -> Self { Self :: new () } # [inline (always)] fn into_inner (self) -> Result < Self :: Inner , Self > { Ok (self) } }
};
}
