// Generated macro for NeverClassifyEos (struct)
macro_rules! Depcrate_classifyNeverClassifyEos {
() => {
// Module: crate::classify
// Provides: {"NeverClassifyEos"}
// Dependencies: {}
# [doc = " A [`ClassifyEos`] type that can be used in [`ClassifyResponse`] implementations that never have"] # [doc = " to classify streaming responses."] # [doc = ""] # [doc = " `NeverClassifyEos` exists only as type.  `NeverClassifyEos` values cannot be constructed."] pub struct NeverClassifyEos < T > { _output_ty : PhantomData < fn () -> T > , _never : Infallible , }
};
}
