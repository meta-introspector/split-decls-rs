// Generated macro for str_to_cdata (function)
macro_rules! Depcrate_formatters_junitstr_to_cdata {
() => {
// Module: crate::formatters::junit
// Provides: {"str_to_cdata"}
// Dependencies: {}
fn str_to_cdata (s : & str) -> String { let escaped_output = s . replace ("]]>" , "]]]]><![CDATA[>") ; let escaped_output = escaped_output . replace ("<?" , "<]]><![CDATA[?") ; let escaped_output = escaped_output . replace ('\n' , "]]>&#xA;<![CDATA[") ; let escaped_output = escaped_output . replace ("<![CDATA[]]>" , "") ; format ! ("<![CDATA[{}]]>" , escaped_output) }
};
}
