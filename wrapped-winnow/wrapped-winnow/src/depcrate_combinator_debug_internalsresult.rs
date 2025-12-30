// Generated macro for result (function)
macro_rules! Depcrate_combinator_debug_internalsresult {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"result"}
// Dependencies: {}
pub (crate) fn result (depth : usize , name : & dyn core :: fmt :: Display , severity : Severity) { let gutter_style = anstyle :: Style :: new () . bold () ; let (call_width , _) = column_widths () ; let call_column = format ! ("{:depth$}| {name}" , "") ; let (status_style , status) = match severity { Severity :: Success => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Green . into ())) , "" ,) , Severity :: Backtrack => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Yellow . into ())) , "backtrack" ,) , Severity :: Cut => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Red . into ())) , "cut" ,) , Severity :: Incomplete => (anstyle :: Style :: new () . fg_color (Some (anstyle :: AnsiColor :: Red . into ())) , "incomplete" ,) , } ; let writer = anstream :: stderr () ; let mut writer = writer . lock () ; let _ = writeln ! (writer , "{status_style}{call_column:call_width$}{status_reset} {gutter_style}|{gutter_reset} {status_style}{status}{status_reset}" , gutter_style = gutter_style . render () , gutter_reset = gutter_style . render_reset () , status_style = status_style . render () , status_reset = status_style . render_reset () ,) ; }
};
}
