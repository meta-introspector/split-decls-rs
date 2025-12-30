// Generated macro for VarZeroVecFormatError (enum)
macro_rules! Depcrate_varzerovec_errorVarZeroVecFormatError {
() => {
// Module: crate::varzerovec::error
// Provides: {"VarZeroVecFormatError"}
// Dependencies: {}
# [derive (Debug)] pub enum VarZeroVecFormatError { # [doc = " The byte buffer was not in the appropriate format for VarZeroVec."] Metadata , # [doc = " One of the values could not be decoded."] Values (crate :: ule :: UleError) , }
};
}
