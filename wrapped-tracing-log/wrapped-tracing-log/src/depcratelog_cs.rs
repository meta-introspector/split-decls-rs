// Generated macro for log_cs (macro)
macro_rules! Depcratelog_cs {
() => {
// Module: crate
// Provides: {"log_cs"}
// Dependencies: {}
macro_rules ! log_cs { ($ level : expr , $ cs : ident , $ meta : ident , $ ty : ident) => { struct $ ty ; static $ cs : $ ty = $ ty ; static $ meta : Metadata <'static > = Metadata :: new ("log event" , "log" , $ level , :: core :: option :: Option :: None , :: core :: option :: Option :: None , :: core :: option :: Option :: None , field :: FieldSet :: new (FIELD_NAMES , identify_callsite ! (&$ cs)) , Kind :: EVENT ,) ; impl callsite :: Callsite for $ ty { fn set_interest (& self , _ : subscriber :: Interest) { } fn metadata (& self) -> &'static Metadata <'static > { &$ meta } } } ; }
};
}
