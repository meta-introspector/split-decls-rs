// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < S : Serializer > Visit for VisitDynamic < '_ , S > { fn visit_named_fields (& mut self , named_values : & NamedValues < '_ >) { let ser = match self { Self :: NamedFields (ser) => ser , Self :: Error (..) => return , Self :: UnnamedFields (..) => { * self = Self :: Error (S :: Error :: custom ("visit_named_fields in unnamed dynamic struct/variant" ,)) ; return ; } } ; for (f , v) in named_values { if let Err (e) = ser . serialize_entry (f . name () , & Serializable (v)) { * self = Self :: Error (e) ; return ; } } } fn visit_unnamed_fields (& mut self , values : & [Value < '_ >]) { let ser = match self { Self :: UnnamedFields (ser) => ser , Self :: Error (..) => return , Self :: NamedFields (..) => { * self = Self :: Error (S :: Error :: custom ("visit_unnamed_fields in named dynamic struct/variant" ,)) ; return ; } } ; for v in values { if let Err (e) = ser . serialize_element (& Serializable (v)) { * self = Self :: Error (e) ; return ; } } } fn visit_entry (& mut self , _ : Value < '_ > , _ : Value < '_ >) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_entry in dynamic struct/variant")) ; } } fn visit_value (& mut self , _ : Value < '_ >) { if ! matches ! (self , Self :: Error (..)) { * self = Self :: Error (S :: Error :: custom ("visit_value in dynamic struct/variant")) ; } } }
};
}
