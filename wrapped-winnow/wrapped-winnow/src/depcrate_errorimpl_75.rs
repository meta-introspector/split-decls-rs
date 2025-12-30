// Generated macro for impl_75 (impl)
macro_rules! Depcrate_errorimpl_75 {
() => {
// Module: crate::error
// Provides: {"impl_75"}
// Dependencies: {}
impl < I : Clone > InputError < I > { # [doc = " Creates a new basic error"] # [inline] pub fn at (input : I) -> Self { Self { input } } # [doc = " Translate the input type"] # [inline] pub fn map_input < I2 : Clone , O : Fn (I) -> I2 > (self , op : O) -> InputError < I2 > { InputError { input : op (self . input) , } } }
};
}
