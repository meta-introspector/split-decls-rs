// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl Debug for Zone < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Zone") . field ("simple" , self . simple ()) . field ("rule" , & self . simple () . final_rule (& self . info . rules)) . field ("name" , & self . name () . chars () . collect :: < String > ()) . field ("region" , & self . region ()) . finish () } }
};
}
