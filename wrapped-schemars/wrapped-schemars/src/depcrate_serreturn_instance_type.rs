// Generated macro for return_instance_type (macro)
macro_rules! Depcrate_serreturn_instance_type {
() => {
// Module: crate::ser
// Provides: {"return_instance_type"}
// Dependencies: {}
macro_rules ! return_instance_type { ($ fn : ident , $ ty : ty , $ instance_type : expr) => { fn $ fn (self , _value : $ ty) -> Result < Self :: Ok , Self :: Error > { Ok (json_schema ! ({ "type" : $ instance_type })) } } ; }
};
}
