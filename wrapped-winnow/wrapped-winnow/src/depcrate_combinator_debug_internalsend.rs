// Generated macro for end (function)
macro_rules! Depcrate_combinator_debug_internalsend {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"end"}
// Dependencies: {}
pub (crate) fn end (depth : usize , name : & dyn core :: fmt :: Display , count : usize , consumed : usize , severity : Severity ,) { let gutter_style = anstyle :: Style :: new () . bold () ; let (call_width , _) = column_widths () ; let count = if 0 < count { format ! (":{count}") } else { "" . to_owned () } ; let call_column = format ! ("{:depth$}< {name}{count}" , "") ; let (status_style , status) = match severity { Severity :: Success => { let style = anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Green . into ())) ; let status = format ! ("+{consumed}") ; (style , status) } Severity :: Backtrack => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Yellow . into ())) , "backtrack" . to_owned () ,) , Severity :: Cut => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Red . into ())) , "cut" . to_owned () ,) , Severity :: Incomplete => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Red . into ())) , "incomplete" . to_owned () ,) , } ; let writer = anstream :: stderr () ; let mut writer = writer . lock () ; let _ = writeln ! (writer , "{status_style}{call_column:call_width$}{status_reset} {gutter_style}|{gutter_reset} {status_style}{status}{status_reset}" , gutter_style = gutter_style . render () , gutter_reset = gutter_style . render_reset () , status_style = status_style . render () , status_reset = status_style . render_reset () ,) ; }
};
}
