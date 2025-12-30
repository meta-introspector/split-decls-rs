// Generated macro for Version (enum)
macro_rules! DepcrateVersion {
() => {
// Module: crate
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Represents the encoding version, used as the first byte in the output."] # [derive (Debug , PartialEq , Eq , FromPrimitive , ToPrimitive)] # [repr (u8)] pub enum Version { Base2 = 0 , Base3 = 1 , }
};
}
