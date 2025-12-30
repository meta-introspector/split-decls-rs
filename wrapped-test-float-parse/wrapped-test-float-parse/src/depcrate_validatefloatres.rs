// Generated macro for FloatRes (enum)
macro_rules! Depcrate_validateFloatRes {
() => {
// Module: crate::validate
// Provides: {"FloatRes"}
// Dependencies: {}
# [doc = " The result of parsing a string to a float type."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum FloatRes < F : Float > { Inf , NegInf , Zero , Nan , # [doc = " A real number with significand and exponent. Value is `sig * 2 ^ exp`."] Real { sig : F :: SInt , exp : i32 , } , }
};
}
