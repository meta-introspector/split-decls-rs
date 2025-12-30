// Generated macro for impl_157 (impl)
macro_rules! Depcrate_error_invalid_format_descriptionimpl_157 {
() => {
// Module: crate::error::invalid_format_description
// Provides: {"impl_157"}
// Dependencies: {}
impl fmt :: Display for InvalidFormatDescription { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use InvalidFormatDescription :: * ; match self { UnclosedOpeningBracket { index } => { write ! (f , "unclosed opening bracket at byte index {index}") } InvalidComponentName { name , index } => { write ! (f , "invalid component name `{name}` at byte index {index}") } InvalidModifier { value , index } => { write ! (f , "invalid modifier `{value}` at byte index {index}") } MissingComponentName { index } => { write ! (f , "missing component name at byte index {index}") } MissingRequiredModifier { name , index } => { write ! (f , "missing required modifier `{name}` for component at byte index {index}") } Expected { what : expected , index , } => { write ! (f , "expected {expected} at byte index {index}") } NotSupported { what , context , index , } => { if context . is_empty () { write ! (f , "{what} is not supported at byte index {index}") } else { write ! (f , "{what} is not supported in {context} at byte index {index}") } } } } }
};
}
