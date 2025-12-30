// Generated macro for ArgsIter (struct)
macro_rules! Depcrate_compiler_argsArgsIter {
() => {
// Module: crate::compiler::args
// Provides: {"ArgsIter"}
// Dependencies: {}
# [doc = " An `Iterator` for parsed arguments"] pub struct ArgsIter < I , T , S > where I : Iterator < Item = OsString > , S : SearchableArgInfo < T > , { arguments : I , arg_info : S , seen_double_dashes : Option < bool > , phantom : PhantomData < T > , }
};
}
