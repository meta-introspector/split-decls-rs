// Generated macro for diff_pretty (function)
macro_rules! Depcrate_framework_graphvizdiff_pretty {
() => {
// Module: crate::framework::graphviz
// Provides: {"diff_pretty"}
// Dependencies: {}
fn diff_pretty < T , C > (new : T , old : T , ctxt : & C) -> String where T : DebugWithContext < C > , { if new == old { return String :: new () ; } let re = regex ! ("\t?\u{001f}([+-])") ; let raw_diff = format ! ("{:#?}" , DebugDiffWithAdapter { new , old , ctxt }) ; let raw_diff = dot :: escape_html (& raw_diff) ; let raw_diff = raw_diff . replace ('\n' , r#"<br align="left"/>"#) ; let mut inside_font_tag = false ; let html_diff = re . replace_all (& raw_diff , | captures : & regex :: Captures < '_ > | { let mut ret = String :: new () ; if inside_font_tag { ret . push_str (r#"</font>"#) ; } let tag = match & captures [1] { "+" => r#"<font color="darkgreen">+"# , "-" => r#"<font color="red">-"# , _ => unreachable ! () , } ; inside_font_tag = true ; ret . push_str (tag) ; ret }) ; let Cow :: Owned (mut html_diff) = html_diff else { return raw_diff ; } ; if inside_font_tag { html_diff . push_str ("</font>") ; } html_diff }
};
}
