// Generated macro for impl_61 (impl)
macro_rules! Depcrate_errorimpl_61 {
() => {
// Module: crate::error
// Provides: {"impl_61"}
// Dependencies: {}
impl < E1 , E2 > ErrorConvert < ErrMode < E2 > > for ErrMode < E1 > where E1 : ErrorConvert < E2 > , { # [inline (always)] fn convert (self) -> ErrMode < E2 > { self . map (| e | e . convert ()) } }
};
}
