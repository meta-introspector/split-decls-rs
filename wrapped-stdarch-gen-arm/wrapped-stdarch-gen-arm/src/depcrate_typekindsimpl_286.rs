// Generated macro for impl_286 (impl)
macro_rules! Depcrate_typekindsimpl_286 {
() => {
// Module: crate::typekinds
// Provides: {"impl_286"}
// Dependencies: {}
impl FromStr for VectorTupleSize { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "2" => Ok (Self :: Two) , "3" => Ok (Self :: Three) , "4" => Ok (Self :: Four) , _ => Err (format ! ("invalid vector tuple size `{s}` provided")) , } } }
};
}
