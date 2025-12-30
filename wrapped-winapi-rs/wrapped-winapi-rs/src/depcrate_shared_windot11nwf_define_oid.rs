// Generated macro for NWF_DEFINE_OID (macro)
macro_rules! Depcrate_shared_windot11NWF_DEFINE_OID {
() => {
// Module: crate::shared::windot11
// Provides: {"NWF_DEFINE_OID"}
// Dependencies: {}
macro_rules ! NWF_DEFINE_OID { ($ Seq : expr , $ o : expr , $ m : expr) => { 0x0E000000 | $ o << 16 | $ m << 8 | $ Seq } ; }
};
}
