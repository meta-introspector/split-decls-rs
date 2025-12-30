// Generated macro for create_default_writer (function)
macro_rules! Depcratecreate_default_writer {
() => {
// Module: crate
// Provides: {"create_default_writer"}
// Dependencies: {}
fn create_default_writer () -> Box < dyn Write + Send > { Box :: new (std :: fs :: File :: create (format ! ("./trace-{}.json" , std :: time :: SystemTime :: UNIX_EPOCH . elapsed () . unwrap () . as_micros ())) . expect ("Failed to create trace file.") ,) }
};
}
