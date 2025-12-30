// Generated macro for impl_389 (impl)
macro_rules! Depcrate_compiler_argsimpl_389 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_389"}
// Dependencies: {}
impl < I , T , S > ArgsIter < I , T , S > where I : Iterator < Item = OsString > , T : ArgumentValue , S : SearchableArgInfo < T > , { # [doc = " Create an `Iterator` for parsed arguments, given an iterator of raw"] # [doc = " `OsString` arguments, and argument descriptions."] pub fn new (arguments : I , arg_info : S) -> Self { # [cfg (debug_assertions)] debug_assert ! (arg_info . check ()) ; ArgsIter { arguments , arg_info , seen_double_dashes : None , phantom : PhantomData , } } pub fn with_double_dashes (mut self) -> Self { self . seen_double_dashes = Some (false) ; self } }
};
}
