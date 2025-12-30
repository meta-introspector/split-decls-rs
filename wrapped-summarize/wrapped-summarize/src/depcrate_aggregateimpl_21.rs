// Generated macro for impl_21 (impl)
macro_rules! Depcrate_aggregateimpl_21 {
() => {
// Module: crate::aggregate
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > From < Event < 'a > > for EventDescription < 'a > { fn from (e : Event < 'a >) -> Self { EventDescription { event_kind : e . event_kind , label : e . label , additional_data : e . additional_data , } } }
};
}
