// Generated macro for start (function)
macro_rules! Depcrate_combinator_debug_internalsstart {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"start"}
// Dependencies: {}
pub (crate) fn start < I : Stream > (depth : usize , name : & dyn core :: fmt :: Display , count : usize , input : & I ,) { let gutter_style = anstyle :: Style :: new () . bold () ; let input_style = anstyle :: Style :: new () . underline () ; let eof_style = anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Cyan . into ())) ; let (call_width , input_width) = column_widths () ; let count = if 0 < count { format ! (":{count}") } else { "" . to_owned () } ; let call_column = format ! ("{:depth$}> {name}{count}" , "") ; let mut debug_slice = format ! ("{:?}" , crate :: util :: from_fn (| f | input . trace (f))) ; let (debug_slice , eof) = if let Some (debug_offset) = debug_slice . char_indices () . enumerate () . find_map (| (pos , (offset , _)) | (input_width <= pos) . then_some (offset)) { debug_slice . truncate (debug_offset) ; let eof = "" ; (debug_slice , eof) } else { let eof = if debug_slice . chars () . count () < input_width { "∅" } else { "" } ; (debug_slice , eof) } ; let writer = anstream :: stderr () ; let mut writer = writer . lock () ; let _ = writeln ! (writer , "{call_column:call_width$} {gutter_style}|{gutter_reset} {input_style}{debug_slice}{input_reset}{eof_style}{eof}{eof_reset}" , gutter_style = gutter_style . render () , gutter_reset = gutter_style . render_reset () , input_style = input_style . render () , input_reset = input_style . render_reset () , eof_style = eof_style . render () , eof_reset = eof_style . render_reset () ,) ; }
};
}
