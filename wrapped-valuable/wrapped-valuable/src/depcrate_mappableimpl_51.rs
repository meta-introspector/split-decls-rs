// Generated macro for impl_51 (impl)
macro_rules! Depcrate_mappableimpl_51 {
() => {
// Module: crate::mappable
// Provides: {"impl_51"}
// Dependencies: {}
impl fmt :: Debug for dyn Mappable + '_ { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct DebugMappable < 'a , 'b > { fmt : fmt :: DebugMap < 'a , 'b > , } impl Visit for DebugMappable < '_ , '_ > { fn visit_entry (& mut self , key : Value < '_ > , value : Value < '_ >) { self . fmt . entry (& key , & value) ; } fn visit_value (& mut self , _ : Value < '_ >) { } } let mut debug = DebugMappable { fmt : fmt . debug_map () , } ; self . visit (& mut debug) ; debug . fmt . finish () } }
};
}
