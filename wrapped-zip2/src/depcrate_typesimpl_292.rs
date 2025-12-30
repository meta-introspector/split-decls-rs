// Generated macro for impl_292 (impl)
macro_rules! Depcrate_typesimpl_292 {
() => {
// Module: crate::types
// Provides: {"impl_292"}
// Dependencies: {}
impl Debug for DateTime { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { if * self == Self :: default () { return f . write_str ("DateTime::default()") ; } f . write_fmt (format_args ! ("DateTime::from_date_and_time({}, {}, {}, {}, {}, {})?" , self . year () , self . month () , self . day () , self . hour () , self . minute () , self . second ())) } }
};
}
