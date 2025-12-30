// Generated macro for impl_926 (impl)
macro_rules! Depcrate_io_buffered_linewriterimpl_926 {
() => {
// Module: crate::io::buffered::linewriter
// Provides: {"impl_926"}
// Dependencies: {}
impl < W : ? Sized + Write > LineWriter < W > { # [doc = " Gets a reference to the underlying writer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fs::File;"] # [doc = " use std::io::LineWriter;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let file = File::create(\"poem.txt\")?;"] # [doc = "     let file = LineWriter::new(file);"] # [doc = ""] # [doc = "     let reference = file.get_ref();"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn get_ref (& self) -> & W { self . inner . get_ref () } }
};
}
