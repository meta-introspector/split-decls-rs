// Generated macro for impl_112 (impl)
macro_rules! Depcrate_treeimpl_112 {
() => {
// Module: crate::tree
// Provides: {"impl_112"}
// Dependencies: {}
impl Event { # [doc = " Returns the event's [`Uuid`]."] # [cfg (feature = "uuid")] pub fn uuid (& self) -> Uuid { self . shared . uuid } # [doc = " Returns the [`DateTime`] that the event occurred at."] # [cfg (feature = "chrono")] pub fn timestamp (& self) -> DateTime < Utc > { self . shared . timestamp } # [doc = " Returns the event's [`Level`]."] pub fn level (& self) -> Level { self . shared . level } # [doc = " Returns the event's message, if there is one."] pub fn message (& self) -> Option < & str > { self . message . as_deref () } # [doc = " Returns the event's [`Tag`], if there is one."] pub fn tag (& self) -> Option < Tag > { self . tag } # [doc = " Returns the event's fields."] pub fn fields (& self) -> & [Field] { & self . shared . fields } }
};
}
