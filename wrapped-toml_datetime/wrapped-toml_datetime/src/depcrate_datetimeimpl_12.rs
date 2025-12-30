// Generated macro for impl_12 (impl)
macro_rules! Depcrate_datetimeimpl_12 {
() => {
// Module: crate::datetime
// Provides: {"impl_12"}
// Dependencies: {}
impl Datetime { # [cfg (feature = "serde")] fn type_name (& self) -> & 'static str { match (self . date . is_some () , self . time . is_some () , self . offset . is_some () ,) { (true , true , true) => "offset datetime" , (true , true , false) => "local datetime" , (true , false , false) => Date :: type_name () , (false , true , false) => Time :: type_name () , _ => unreachable ! ("unsupported datetime combination") , } } }
};
}
