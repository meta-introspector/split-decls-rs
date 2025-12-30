// Generated macro for DeflateConfig (struct)
macro_rules! Depcrate_deflateDeflateConfig {
() => {
// Module: crate::deflate
// Provides: {"DeflateConfig"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "__internal-fuzz" , derive (arbitrary :: Arbitrary))] pub struct DeflateConfig { pub level : i32 , pub method : Method , pub window_bits : i32 , pub mem_level : i32 , pub strategy : Strategy , }
};
}
