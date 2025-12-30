// Generated macro for impl_830 (impl)
macro_rules! Depcrate_settings_hooksimpl_830 {
() => {
// Module: crate::settings::hooks
// Provides: {"impl_830"}
// Dependencies: {}
impl std :: fmt :: Debug for Hooks { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { fn hook_status < T > (val : & Option < T >) -> & 'static str { match val { Some (_) => "enabled" , None => "disabled" , } } f . debug_struct ("Hooks") . field ("connection_hook" , & hook_status (& self . connection_hook)) . finish () } }
};
}
