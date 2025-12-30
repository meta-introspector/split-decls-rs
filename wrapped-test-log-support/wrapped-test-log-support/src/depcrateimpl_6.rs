// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Log for Logger { fn enabled (& self , meta : & Metadata) -> bool { for (target , level) in & self . filters { if meta . target () . starts_with (target) { return meta . level () <= * level ; } } false } fn log (& self , record : & Record) { let line = format ! ("{}" , record . args ()) ; println ! ("{:<5} {} {}" , record . level () , record . target () , line) ; if let Ok (mut last) = self . state . last_log . lock () { * last = Some (line) ; } } fn flush (& self) { } }
};
}
