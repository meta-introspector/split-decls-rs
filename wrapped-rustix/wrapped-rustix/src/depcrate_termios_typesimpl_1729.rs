// Generated macro for impl_1729 (impl)
macro_rules! Depcrate_termios_typesimpl_1729 {
() => {
// Module: crate::termios::types
// Provides: {"impl_1729"}
// Dependencies: {}
impl core :: fmt :: Debug for Termios { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut d = f . debug_struct ("Termios") ; d . field ("input_modes" , & self . input_modes) ; d . field ("output_modes" , & self . output_modes) ; d . field ("control_modes" , & self . control_modes) ; d . field ("local_modes" , & self . local_modes) ; # [cfg (any (linux_like , target_env = "newlib" , target_os = "fuchsia" , target_os = "haiku" , target_os = "redox"))] { d . field ("line_discipline" , & SpecialCode (self . line_discipline)) ; } d . field ("special_codes" , & self . special_codes) ; d . field ("input_speed" , & self . input_speed ()) ; d . field ("output_speed" , & self . output_speed ()) ; d . finish () } }
};
}
