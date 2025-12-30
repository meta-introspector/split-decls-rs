// Generated macro for print_diff (function)
macro_rules! Depcrate_rustfmt_diffprint_diff {
() => {
// Module: crate::rustfmt_diff
// Provides: {"print_diff"}
// Dependencies: {}
pub (crate) fn print_diff < F > (diff : Vec < Mismatch > , get_section_title : F , config : & Config) where F : Fn (u32) -> String , { let color = config . color () ; let line_terminator = if config . verbose () == Verbosity :: Verbose { "⏎" } else { "" } ; let mut writer = OutputWriter :: new (color) ; for mismatch in diff { let title = get_section_title (mismatch . line_number_orig) ; writer . writeln (& title , None) ; for line in mismatch . lines { match line { DiffLine :: Context (ref str) => { writer . writeln (& format ! (" {str}{line_terminator}") , None) } DiffLine :: Expected (ref str) => writer . writeln (& format ! ("+{str}{line_terminator}") , Some (term :: color :: GREEN) ,) , DiffLine :: Resulting (ref str) => { writer . writeln (& format ! ("-{str}{line_terminator}") , Some (term :: color :: RED)) } } } } }
};
}
