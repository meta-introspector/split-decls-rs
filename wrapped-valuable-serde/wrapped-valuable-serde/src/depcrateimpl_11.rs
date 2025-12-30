// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < S : Serializer > Visit for VisitList < '_ , S > { fn visit_value (& mut self , value : Value < '_ >) { if let Self :: Serializer (ser) = self { if let Err (e) = ser . serialize_element (& Serializable (value)) { * self = Self :: Error (e) ; } } } fn visit_entry (& mut self , _ : Value < '_ > , _ : Value < '_ >) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_entry in list")) ; } } fn visit_named_fields (& mut self , _ : & NamedValues < '_ >) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_named_fields in list")) ; } } fn visit_unnamed_fields (& mut self , _ : & [Value < '_ >]) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_unnamed_fields in list")) ; } } }
};
}
