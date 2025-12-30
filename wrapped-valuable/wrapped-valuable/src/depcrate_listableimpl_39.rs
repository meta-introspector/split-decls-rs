// Generated macro for impl_39 (impl)
macro_rules! Depcrate_listableimpl_39 {
() => {
// Module: crate::listable
// Provides: {"impl_39"}
// Dependencies: {}
impl fmt :: Debug for dyn Listable + '_ { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct DebugListable < 'a , 'b > { fmt : fmt :: DebugList < 'a , 'b > , } impl Visit for DebugListable < '_ , '_ > { fn visit_value (& mut self , value : Value < '_ >) { self . fmt . entry (& value) ; } } let mut debug = DebugListable { fmt : fmt . debug_list () , } ; self . visit (& mut debug) ; debug . fmt . finish () } }
};
}
