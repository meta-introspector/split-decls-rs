// Generated macro for impl_248 (impl)
macro_rules! Depcrate_debugimpl_248 {
() => {
// Module: crate::debug
// Provides: {"impl_248"}
// Dependencies: {}
# [cfg (feature = "component")] impl std :: fmt :: Debug for crate :: Component { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{} " , self . label ()) ? ; if let Some (temperature) = self . temperature () { write ! (f , "temperature: {temperature}°C (") ? ; } else { f . write_str ("temperature: unknown (") ? ; } if let Some (max) = self . max () { write ! (f , "max: {max}°C / ") ? ; } else { f . write_str ("max: unknown / ") ? ; } if let Some (critical) = self . critical () { write ! (f , "critical: {critical}°C)") } else { f . write_str ("critical: unknown)") } } }
};
}
