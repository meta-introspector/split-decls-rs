// Generated macro for tempfile_in (function)
macro_rules! Depcrate_filetempfile_in {
() => {
// Module: crate::file
// Provides: {"tempfile_in"}
// Dependencies: {}
# [doc = " Create a new temporary file in the specified directory. Also see [`tempfile`]."] # [doc = ""] # [doc = " # Security"] # [doc = ""] # [doc = " This variant is secure/reliable in the presence of a pathological temporary file cleaner."] # [doc = " If the temporary file isn't created in [`env::temp_dir()`] then temporary file cleaners aren't an issue."] # [doc = ""] # [doc = " # Resource Leaking"] # [doc = ""] # [doc = " The temporary file will be automatically removed by the OS when the last handle to it is closed."] # [doc = " This doesn't rely on Rust destructors being run, so will (almost) never fail to clean up the temporary file."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the file can not be created, `Err` is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use tempfile::tempfile_in;"] # [doc = " use std::io::Write;"] # [doc = ""] # [doc = " // Create a file inside of the current working directory"] # [doc = " let mut file = tempfile_in(\"./\")?;"] # [doc = ""] # [doc = " writeln!(file, \"Brian was here. Briefly.\")?;"] # [doc = " # Ok::<(), std::io::Error>(())"] # [doc = " ```"] pub fn tempfile_in < P : AsRef < Path > > (dir : P) -> io :: Result < File > { imp :: create (dir . as_ref ()) }
};
}
