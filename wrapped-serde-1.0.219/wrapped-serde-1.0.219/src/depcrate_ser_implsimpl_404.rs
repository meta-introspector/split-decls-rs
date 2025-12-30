// Generated macro for impl_404 (impl)
macro_rules! Depcrate_ser_implsimpl_404 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_404"}
// Dependencies: {}
impl < T , E > Serialize for Result < T , E > where T : Serialize , E : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match * self { Result :: Ok (ref value) => serializer . serialize_newtype_variant ("Result" , 0 , "Ok" , value) , Result :: Err (ref value) => { serializer . serialize_newtype_variant ("Result" , 1 , "Err" , value) } } } }
};
}
