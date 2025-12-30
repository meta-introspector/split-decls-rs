// Generated macro for impl_94 (impl)
macro_rules! Depcrate_tuplableimpl_94 {
() => {
// Module: crate::tuplable
// Provides: {"impl_94"}
// Dependencies: {}
impl fmt :: Debug for dyn Tuplable + '_ { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . definition () . is_unit () { () . fmt (fmt) } else { struct DebugTuple < 'a , 'b > { fmt : fmt :: DebugTuple < 'a , 'b > , } impl Visit for DebugTuple < '_ , '_ > { fn visit_unnamed_fields (& mut self , values : & [Value < '_ >]) { for value in values { self . fmt . field (value) ; } } fn visit_value (& mut self , _ : Value < '_ >) { unimplemented ! () } } let mut debug = DebugTuple { fmt : fmt . debug_tuple ("") , } ; self . visit (& mut debug) ; debug . fmt . finish () } } }
};
}
