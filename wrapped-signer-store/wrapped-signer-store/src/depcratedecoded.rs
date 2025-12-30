// Generated macro for Decoded (enum)
macro_rules! DepcrateDecoded {
() => {
// Module: crate
// Provides: {"Decoded"}
// Dependencies: {}
# [doc = " Represents the result of a decoding operation."] # [derive (Debug , PartialEq , Eq)] pub enum Decoded { # [doc = " A single vector from Base2 decoding."] Base2 (BitVec < u8 , Lsb0 >) , # [doc = " Two vectors from Base3 decoding."] Base3 (BitVec < u8 , Lsb0 > , BitVec < u8 , Lsb0 >) , }
};
}
