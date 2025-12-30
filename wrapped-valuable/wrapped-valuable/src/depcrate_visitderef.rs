// Generated macro for deref (macro)
macro_rules! Depcrate_visitderef {
() => {
// Module: crate::visit
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Visit > Visit for $ ty { fn visit_value (& mut self , value : Value <'_ >) { T :: visit_value (& mut ** self , value) } fn visit_named_fields (& mut self , named_values : & NamedValues <'_ >) { T :: visit_named_fields (& mut ** self , named_values) } fn visit_unnamed_fields (& mut self , values : & [Value <'_ >]) { T :: visit_unnamed_fields (& mut ** self , values) } fn visit_primitive_slice (& mut self , slice : Slice <'_ >) { T :: visit_primitive_slice (& mut ** self , slice) } fn visit_entry (& mut self , key : Value <'_ >, value : Value <'_ >) { T :: visit_entry (& mut ** self , key , value) } }) * } ; }
};
}
