// Generated macro for impl_506 (impl)
macro_rules! Depcrate_stream_tokenimpl_506 {
() => {
// Module: crate::stream::token
// Provides: {"impl_506"}
// Dependencies: {}
impl < 't , T > StreamIsPartial for TokenSlice < 't , T > where T : core :: fmt :: Debug + Clone , { type PartialState = < & 't [T] as StreamIsPartial > :: PartialState ; # [inline] fn complete (& mut self) -> Self :: PartialState { # ! [allow (clippy :: semicolon_if_nothing_returned)] self . input . complete () } # [inline] fn restore_partial (& mut self , state : Self :: PartialState) { self . input . restore_partial (state) ; } # [inline (always)] fn is_partial_supported () -> bool { < & [T] as StreamIsPartial > :: is_partial_supported () } # [inline (always)] fn is_partial (& self) -> bool { self . input . is_partial () } }
};
}
