// Generated macro for impl_79 (impl)
macro_rules! Depcrate_structableimpl_79 {
() => {
// Module: crate::structable
// Provides: {"impl_79"}
// Dependencies: {}
impl fmt :: Debug for dyn Structable + '_ { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let def = self . definition () ; if def . fields () . is_named () { struct DebugStruct < 'a , 'b > { fmt : fmt :: DebugStruct < 'a , 'b > , } let mut debug = DebugStruct { fmt : fmt . debug_struct (def . name ()) , } ; impl Visit for DebugStruct < '_ , '_ > { fn visit_named_fields (& mut self , named_values : & NamedValues < '_ >) { for (field , value) in named_values { self . fmt . field (field . name () , value) ; } } fn visit_value (& mut self , _ : Value < '_ >) { unreachable ! () } } self . visit (& mut debug) ; debug . fmt . finish () } else { struct DebugStruct < 'a , 'b > { fmt : fmt :: DebugTuple < 'a , 'b > , } let mut debug = DebugStruct { fmt : fmt . debug_tuple (def . name ()) , } ; impl Visit for DebugStruct < '_ , '_ > { fn visit_unnamed_fields (& mut self , values : & [Value < '_ >]) { for value in values { self . fmt . field (value) ; } } fn visit_value (& mut self , _ : Value < '_ >) { unreachable ! () ; } } self . visit (& mut debug) ; debug . fmt . finish () } } }
};
}
