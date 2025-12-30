// Generated macro for impl_964 (impl)
macro_rules! Depcrate_serdeimpl_964 {
() => {
// Module: crate::serde
// Provides: {"impl_964"}
// Dependencies: {}
impl Serialize for Duration { # [inline] fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { # [cfg (feature = "serde-human-readable")] if serializer . is_human_readable () { return serializer . collect_str (& format_args ! ("{}{}.{:>09}" , if self . is_negative () { "-" } else { "" } , self . whole_seconds () . unsigned_abs () , self . subsec_nanoseconds () . abs () ,)) ; } (self . whole_seconds () , self . subsec_nanoseconds ()) . serialize (serializer) } }
};
}
