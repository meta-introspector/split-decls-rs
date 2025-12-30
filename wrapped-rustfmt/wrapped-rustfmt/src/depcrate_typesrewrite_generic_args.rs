// Generated macro for rewrite_generic_args (function)
macro_rules! Depcrate_typesrewrite_generic_args {
() => {
// Module: crate::types
// Provides: {"rewrite_generic_args"}
// Dependencies: {}
fn rewrite_generic_args (gen_args : & ast :: GenericArgs , context : & RewriteContext < '_ > , shape : Shape , span : Span ,) -> RewriteResult { match gen_args { ast :: GenericArgs :: AngleBracketed (ref data) => { if data . args . is_empty () { Ok ("" . to_owned ()) } else { let args = data . args . iter () . map (| x | match x { ast :: AngleBracketedArg :: Arg (generic_arg) => { SegmentParam :: from_generic_arg (generic_arg) } ast :: AngleBracketedArg :: Constraint (constraint) => { SegmentParam :: Binding (constraint) } }) . collect :: < Vec < _ > > () ; overflow :: rewrite_with_angle_brackets (context , "" , args . iter () , shape , span) } } ast :: GenericArgs :: Parenthesized (ref data) => format_function_type (data . inputs . iter () . map (| x | & * * x) , & data . output , false , data . span , context , shape ,) , ast :: GenericArgs :: ParenthesizedElided (..) => Ok ("(..)" . to_owned ()) , } }
};
}
