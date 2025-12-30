// Generated macro for impl_326 (impl)
macro_rules! Depcrate_config_optionsimpl_326 {
() => {
// Module: crate::config::options
// Provides: {"impl_326"}
// Dependencies: {}
impl Density { pub fn to_list_tactic (self , len : usize) -> ListTactic { match self { Density :: Compressed => ListTactic :: Mixed , Density :: Tall => ListTactic :: HorizontalVertical , Density :: Vertical if len == 1 => ListTactic :: Horizontal , Density :: Vertical => ListTactic :: Vertical , } } }
};
}
