// Generated macro for impl_96 (impl)
macro_rules! Depcrate_spanimpl_96 {
() => {
// Module: crate::span
// Provides: {"impl_96"}
// Dependencies: {}
impl Inner { # [doc = " Indicates that the span with the given ID has an indirect causal"] # [doc = " relationship with this span."] # [doc = ""] # [doc = " This relationship differs somewhat from the parent-child relationship: a"] # [doc = " span may have any number of prior spans, rather than a single one; and"] # [doc = " spans are not considered to be executing _inside_ of the spans they"] # [doc = " follow from. This means that a span may close even if subsequent spans"] # [doc = " that follow from it are still open, and time spent inside of a"] # [doc = " subsequent span should not be included in the time its precedents were"] # [doc = " executing. This is used to model causal relationships such as when a"] # [doc = " single future spawns several related background tasks, et cetera."] # [doc = ""] # [doc = " If this span is disabled, this function will do nothing. Otherwise, it"] # [doc = " returns `Ok(())` if the other span was added as a precedent of this"] # [doc = " span, or an error if this was not possible."] fn follows_from (& self , from : & Id) { self . subscriber . record_follows_from (& self . id , from) } # [doc = " Returns the span's ID."] fn id (& self) -> Id { self . id . clone () } fn record (& self , values : & Record < '_ >) { self . subscriber . record (& self . id , values) } fn new (id : Id , subscriber : & Dispatch) -> Self { Inner { id , subscriber : subscriber . clone () , } } }
};
}
