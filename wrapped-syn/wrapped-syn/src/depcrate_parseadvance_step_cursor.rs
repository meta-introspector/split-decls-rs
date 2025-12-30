// Generated macro for advance_step_cursor (function)
macro_rules! Depcrate_parseadvance_step_cursor {
() => {
// Module: crate::parse
// Provides: {"advance_step_cursor"}
// Dependencies: {}
pub (crate) fn advance_step_cursor < 'c , 'a > (proof : StepCursor < 'c , 'a > , to : Cursor < 'c >) -> Cursor < 'a > { let _ = proof ; unsafe { mem :: transmute :: < Cursor < 'c > , Cursor < 'a > > (to) } }
};
}
