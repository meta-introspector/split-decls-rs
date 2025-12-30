// Generated macro for expect_str (function)
macro_rules! Depcrate_is_serialize_strexpect_str {
() => {
// Module: crate::is_serialize_str
// Provides: {"expect_str"}
// Dependencies: {}
pub fn expect_str < T > (value : & T , expected_str : & 'static str) -> Result < () , Unexpected > where T : ? Sized + Serialize , { match value . serialize (Serializer { expected_str }) { Err (SerializerState :: Ok) => Ok (()) , Err (SerializerState :: UnexpectedStr (string)) => Err (Unexpected :: Str (string)) , Err (SerializerState :: UnexpectedKind) => Err (Unexpected :: NonStr) , # [allow (unreachable_patterns)] Ok (void) => match void { } , } }
};
}
