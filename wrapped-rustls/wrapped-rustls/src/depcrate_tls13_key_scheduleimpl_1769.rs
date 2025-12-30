// Generated macro for impl_1769 (impl)
macro_rules! Depcrate_tls13_key_scheduleimpl_1769 {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"impl_1769"}
// Dependencies: {}
impl Exporter for KeyScheduleExporter { fn derive (& self , label : & [u8] , context : Option < & [u8] > , out : & mut [u8]) -> Result < () , Error > { self . ks . export_keying_material (& self . current_exporter_secret , label , context , out) } }
};
}
