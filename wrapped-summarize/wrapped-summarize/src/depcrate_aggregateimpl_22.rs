// Generated macro for impl_22 (impl)
macro_rules! Depcrate_aggregateimpl_22 {
() => {
// Module: crate::aggregate
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for EventDescription < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . event_kind [..] { QUERY_EVENT_KIND | GENERIC_ACTIVITY_EVENT_KIND => { } _ => write ! (f , "{} " , self . event_kind) ? , } write ! (f , "`{}(" , self . label) ? ; for (i , arg) in self . additional_data . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{}" , arg) ? ; } write ! (f , ")`") } }
};
}
