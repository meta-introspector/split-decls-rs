// Generated macro for impl_106 (impl)
macro_rules! Depcrate_layerimpl_106 {
() => {
// Module: crate::layer
// Provides: {"impl_106"}
// Dependencies: {}
impl MockLayer { fn check_event_scope < C > (& self , current_scope : Option < tracing_subscriber :: registry :: Scope < '_ , C > > , expected_scope : & mut [ExpectedSpan] ,) where C : for < 'lookup > tracing_subscriber :: registry :: LookupSpan < 'lookup > , { let mut current_scope = current_scope . into_iter () . flatten () ; let mut i = 0 ; for (expected , actual) in expected_scope . iter_mut () . zip (& mut current_scope) { println ! ("[{}] event_scope[{}] actual={} ({:?}); expected={}" , self . name , i , actual . name () , actual . id () , expected) ; expected . check (& (& actual) . into () , format_args ! ("the {}th span in the event's scope to be" , i) , & self . name ,) ; i += 1 ; } let remaining_expected = & expected_scope [i ..] ; assert ! (remaining_expected . is_empty () , "\n[{}] did not observe all expected spans in event scope!\n[{}] missing: {:#?}" , self . name , self . name , remaining_expected ,) ; assert ! (current_scope . next () . is_none () , "\n[{}] did not expect all spans in the actual event scope!" , self . name ,) ; } }
};
}
