// Generated macro for impl_1109 (impl)
macro_rules! Depcrate_io_pipeimpl_1109 {
() => {
// Module: crate::io::pipe
// Provides: {"impl_1109"}
// Dependencies: {}
impl PipeWriter { # [doc = " Creates a new [`PipeWriter`] instance that shares the same underlying file description."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[cfg(miri)] fn main() {}"] # [doc = " # #[cfg(not(miri))]"] # [doc = " # fn main() -> std::io::Result<()> {"] # [doc = " use std::process::Command;"] # [doc = " use std::io::{pipe, Read};"] # [doc = " let (mut reader, writer) = pipe()?;"] # [doc = ""] # [doc = " // Spawn a process that writes to stdout and stderr."] # [doc = " let mut peer = Command::new(\"bash\")"] # [doc = "     .args(["] # [doc = "         \"-c\","] # [doc = "         \"echo -n foo\\n\\"] # [doc = "          echo -n bar >&2\""] # [doc = "     ])"] # [doc = "     .stdout(writer.try_clone()?)"] # [doc = "     .stderr(writer)"] # [doc = "     .spawn()?;"] # [doc = ""] # [doc = " // Read and check the result."] # [doc = " let mut msg = String::new();"] # [doc = " reader.read_to_string(&mut msg)?;"] # [doc = " assert_eq!(&msg, \"foobar\");"] # [doc = ""] # [doc = " peer.wait()?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [stable (feature = "anonymous_pipe" , since = "1.87.0")] pub fn try_clone (& self) -> io :: Result < Self > { self . 0 . try_clone () . map (Self) } }
};
}
