// Generated macro for CheckError (struct)
macro_rules! Depcrate_validateCheckError {
() => {
// Module: crate::validate
// Provides: {"CheckError"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct CheckError { pub fail : CheckFailure , # [doc = " String for which parsing was attempted."] pub input : Box < str > , # [doc = " The parsed & decomposed `FloatRes`, already stringified so we don't need generics here."] pub float_res : Box < str > , }
};
}
