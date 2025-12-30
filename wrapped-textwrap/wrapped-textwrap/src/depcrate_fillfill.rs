// Generated macro for fill (function)
macro_rules! Depcrate_fillfill {
() => {
// Module: crate::fill
// Provides: {"fill"}
// Dependencies: {}
# [doc = " Fill a line of text at a given width."] # [doc = ""] # [doc = " The result is a [`String`], complete with newlines between each"] # [doc = " line. Use [`wrap()`] if you need access to the individual lines."] # [doc = ""] # [doc = " The easiest way to use this function is to pass an integer for"] # [doc = " `width_or_options`:"] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::fill;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     fill(\"Memory safety without garbage collection.\", 15),"] # [doc = "     \"Memory safety\\nwithout garbage\\ncollection.\""] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " If you need to customize the wrapping, you can pass an [`Options`]"] # [doc = " instead of an `usize`:"] # [doc = ""] # [doc = " ```"] # [doc = " use textwrap::{fill, Options};"] # [doc = ""] # [doc = " let options = Options::new(15)"] # [doc = "     .initial_indent(\"- \")"] # [doc = "     .subsequent_indent(\"  \");"] # [doc = " assert_eq!("] # [doc = "     fill(\"Memory safety without garbage collection.\", &options),"] # [doc = "     \"- Memory safety\\n  without\\n  garbage\\n  collection.\""] # [doc = " );"] # [doc = " ```"] pub fn fill < 'a , Opt > (text : & str , width_or_options : Opt) -> String where Opt : Into < Options < 'a > > , { let options = width_or_options . into () ; if text . len () < options . width && ! text . contains ('\n') && options . initial_indent . is_empty () { String :: from (text . trim_end_matches (' ')) } else { fill_slow_path (text , options) } }
};
}
