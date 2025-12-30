// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < S : Serializer > Visit for VisitMap < '_ , S > { fn visit_entry (& mut self , key : Value < '_ > , value : Value < '_ >) { if let Self :: Serializer (ser) = self { if let Err (e) = ser . serialize_entry (& Serializable (key) , & Serializable (value)) { * self = Self :: Error (e) ; } } } fn visit_value (& mut self , _ : Value < '_ >) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_value in map")) ; } } fn visit_named_fields (& mut self , _ : & NamedValues < '_ >) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_named_fields in map")) ; } } fn visit_unnamed_fields (& mut self , _ : & [Value < '_ >]) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_unnamed_fields in map")) ; } } }
};
}
