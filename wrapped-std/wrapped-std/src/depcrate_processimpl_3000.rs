// Generated macro for impl_3000 (impl)
macro_rules! Depcrate_processimpl_3000 {
() => {
// Module: crate::process
// Provides: {"impl_3000"}
// Dependencies: {}
# [stable (feature = "stdio_from_stdio" , since = "1.74.0")] impl From < io :: Stdout > for Stdio { # [doc = " Redirect command stdout/stderr to our stdout"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(exit_status_error)]"] # [doc = " use std::io;"] # [doc = " use std::process::Command;"] # [doc = ""] # [doc = " # fn test() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " let output = Command::new(\"whoami\")"] # [doc = "     .stdout(io::stdout())"] # [doc = "     .output()?;"] # [doc = " output.status.exit_ok()?;"] # [doc = " assert!(output.stdout.is_empty());"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " #"] # [doc = " # if cfg!(all(unix, not(target_os = \"android\"))) {"] # [doc = " #     test().unwrap();"] # [doc = " # }"] # [doc = " ```"] fn from (inherit : io :: Stdout) -> Stdio { Stdio :: from_inner (inherit . into ()) } }
};
}
