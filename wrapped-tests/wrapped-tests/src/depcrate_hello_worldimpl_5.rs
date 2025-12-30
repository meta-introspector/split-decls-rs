// Generated macro for impl_5 (impl)
macro_rules! Depcrate_hello_worldimpl_5 {
() => {
// Module: crate::hello_world
// Provides: {"impl_5"}
// Dependencies: {}
impl Valuable for HelloWorld { fn as_value (& self) -> Value < '_ > { Value :: Structable (self) } fn visit (& self , visit : & mut dyn Visit) { visit . visit_named_fields (& NamedValues :: new (HELLO_WORLD_FIELDS , & [Value :: U32 (self . id)] ,)) ; } }
};
}
