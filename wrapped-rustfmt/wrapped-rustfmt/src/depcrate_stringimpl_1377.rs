// Generated macro for impl_1377 (impl)
macro_rules! Depcrate_stringimpl_1377 {
() => {
// Module: crate::string
// Provides: {"impl_1377"}
// Dependencies: {}
impl < 'a > StringFormat < 'a > { pub (crate) fn new (shape : Shape , config : & 'a Config) -> StringFormat < 'a > { StringFormat { opener : "\"" , closer : "\"" , line_start : " " , line_end : "\\" , shape , trim_end : false , config , } } # [doc = " Returns the maximum number of graphemes that is possible on a line while taking the"] # [doc = " indentation into account."] # [doc = ""] # [doc = " If we cannot put at least a single character per line, the rewrite won't succeed."] fn max_width_with_indent (& self) -> Option < usize > { Some (self . shape . width . checked_sub (self . opener . len () + self . line_end . len () + 1) ? + 1 ,) } # [doc = " Like max_width_with_indent but the indentation is not subtracted."] # [doc = " This allows to fit more graphemes from the string on a line when"] # [doc = " SnippetState::EndWithLineFeed."] fn max_width_without_indent (& self) -> Option < usize > { self . config . max_width () . checked_sub (self . line_end . len ()) } }
};
}
