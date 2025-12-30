// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < S > Visit for SerdeMapVisitor < S > where S : SerializeMap , { # [cfg (all (tracing_unstable , feature = "valuable"))] # [cfg_attr (docsrs , doc (cfg (all (tracing_unstable , feature = "valuable"))))] fn record_value (& mut self , field : & Field , value : valuable_crate :: Value < '_ >) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & valuable_serde :: Serializable :: new (value)) ; } } fn record_bool (& mut self , field : & Field , value : bool) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & value) } } fn record_debug (& mut self , field : & Field , value : & dyn fmt :: Debug) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & format_args ! ("{:?}" , value)) } } fn record_u64 (& mut self , field : & Field , value : u64) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & value) } } fn record_i64 (& mut self , field : & Field , value : i64) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & value) } } fn record_f64 (& mut self , field : & Field , value : f64) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & value) } } fn record_str (& mut self , field : & Field , value : & str) { if self . state . is_ok () { self . state = self . serializer . serialize_entry (field . name () , & value) } } }
};
}
