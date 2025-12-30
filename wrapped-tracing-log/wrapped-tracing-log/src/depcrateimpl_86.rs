// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl Visit for LogVisitor < '_ > { fn record_debug (& mut self , _field : & Field , _value : & dyn fmt :: Debug) { } fn record_u64 (& mut self , field : & Field , value : u64) { if field == & self . fields . line { self . line = Some (value) ; } } fn record_str (& mut self , field : & Field , value : & str) { unsafe { if field == & self . fields . file { self . file = Some (& * (value as * const _)) ; } else if field == & self . fields . target { self . target = Some (& * (value as * const _)) ; } else if field == & self . fields . module { self . module_path = Some (& * (value as * const _)) ; } } } }
};
}
