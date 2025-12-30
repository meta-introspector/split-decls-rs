// Generated macro for Synom (trait)
macro_rules! DepcrateSynom {
() => {
// Module: crate
// Provides: {"Synom"}
// Dependencies: {}
pub trait Synom : Sized { fn parse (input : Cursor) -> PResult < Self > ; fn description () -> Option < & 'static str > { None } }
};
}
