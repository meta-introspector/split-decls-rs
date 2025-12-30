// Generated macro for Error (struct)
macro_rules! Depcrate_de_errorError {
() => {
// Module: crate::de::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors that can occur when deserializing a type."] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub struct Error { message : String , input : Option < alloc :: sync :: Arc < str > > , keys : Vec < String > , span : Option < core :: ops :: Range < usize > > , }
};
}
