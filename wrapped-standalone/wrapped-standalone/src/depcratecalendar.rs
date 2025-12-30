// Generated macro for calendar (function)
macro_rules! Depcratecalendar {
() => {
// Module: crate
// Provides: {"calendar"}
// Dependencies: {}
# [test] fn calendar () -> windows_core :: Result < () > { use b_calendar :: * ; let calendar = Calendar :: new () ? ; let year = calendar . Year () ? ; calendar . SetYear (year) ? ; Ok (()) }
};
}
