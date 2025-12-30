// Generated macro for impl_14 (impl)
macro_rules! Depcrate_enumerableimpl_14 {
() => {
// Module: crate::enumerable
// Provides: {"impl_14"}
// Dependencies: {}
impl fmt :: Debug for dyn Enumerable + '_ { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let variant = self . variant () ; # [cfg (feature = "alloc")] let name = format ! ("{}::{}" , self . definition () . name () , variant . name ()) ; # [cfg (not (feature = "alloc"))] let name = variant . name () ; if variant . is_named_fields () { struct DebugEnum < 'a , 'b > { fmt : fmt :: DebugStruct < 'a , 'b > , } let mut debug = DebugEnum { fmt : fmt . debug_struct (& name) , } ; impl Visit for DebugEnum < '_ , '_ > { fn visit_named_fields (& mut self , named_values : & NamedValues < '_ >) { for (field , value) in named_values { self . fmt . field (field . name () , value) ; } } fn visit_value (& mut self , _ : Value < '_ >) { unreachable ! () ; } } self . visit (& mut debug) ; debug . fmt . finish () } else { struct DebugEnum < 'a , 'b > { fmt : fmt :: DebugTuple < 'a , 'b > , } let mut debug = DebugEnum { fmt : fmt . debug_tuple (& name) , } ; impl Visit for DebugEnum < '_ , '_ > { fn visit_unnamed_fields (& mut self , values : & [Value < '_ >]) { for value in values { self . fmt . field (value) ; } } fn visit_value (& mut self , _ : Value < '_ >) { unreachable ! () ; } } self . visit (& mut debug) ; debug . fmt . finish () } } }
};
}
