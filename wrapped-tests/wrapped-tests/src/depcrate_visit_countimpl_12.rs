// Generated macro for impl_12 (impl)
macro_rules! Depcrate_visit_countimpl_12 {
() => {
// Module: crate::visit_count
// Provides: {"impl_12"}
// Dependencies: {}
impl Visit for VisitCount { fn visit_value (& mut self , _ : Value < '_ >) { self . visit_value += 1 ; } fn visit_named_fields (& mut self , _ : & NamedValues < '_ >) { self . visit_named_fields += 1 ; } fn visit_unnamed_fields (& mut self , _ : & [Value < '_ >]) { self . visit_unnamed_fields += 1 ; } fn visit_primitive_slice (& mut self , _ : Slice < '_ >) { self . visit_primitive_slice += 1 ; } fn visit_entry (& mut self , _ : Value < '_ > , _ : Value < '_ >) { self . visit_entry += 1 ; } }
};
}
