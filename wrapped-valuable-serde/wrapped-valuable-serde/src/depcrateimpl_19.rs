// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < S : Serializer > Visit for VisitStaticTuple < S > { fn visit_unnamed_fields (& mut self , values : & [Value < '_ >]) { let mut ser = match mem :: replace (self , Self :: Tmp) { Self :: Start (ser) => ser , mut res @ Self :: End (..) => { if matches ! (res , Self :: End (Ok (..))) { res = Self :: End (Err (S :: Error :: custom ("visit_unnamed_fields called multiple times in static tuple" ,))) ; } * self = res ; return ; } _ => unreachable ! () , } ; for v in values { if let Err (e) = ser . serialize_element (& Serializable (v)) { * self = Self :: End (Err (e)) ; return ; } } * self = Self :: End (ser . end ()) ; } fn visit_named_fields (& mut self , _ : & NamedValues < '_ >) { if ! matches ! (self , Self :: End (Err (..))) { * self = Self :: End (Err (S :: Error :: custom ("visit_named_fields in tuple"))) ; } } fn visit_entry (& mut self , _ : Value < '_ > , _ : Value < '_ >) { if ! matches ! (self , Self :: End (Err (..))) { * self = Self :: End (Err (S :: Error :: custom ("visit_entry in tuple"))) ; } } fn visit_value (& mut self , _ : Value < '_ >) { if ! matches ! (self , Self :: End (Err (..))) { * self = Self :: End (Err (S :: Error :: custom ("visit_value in tuple"))) ; } } }
};
}
