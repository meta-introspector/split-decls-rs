// Generated macro for WriterPanicked (struct)
macro_rules! Depcrate_io_buffered_bufwriterWriterPanicked {
() => {
// Module: crate::io::buffered::bufwriter
// Provides: {"WriterPanicked"}
// Dependencies: {}
# [stable (feature = "bufwriter_into_parts" , since = "1.56.0")] # [doc = " Error returned for the buffered data from `BufWriter::into_parts`, when the underlying"] # [doc = " writer has previously panicked.  Contains the (possibly partly written) buffered data."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::{self, BufWriter, Write};"] # [doc = " use std::panic::{catch_unwind, AssertUnwindSafe};"] # [doc = ""] # [doc = " struct PanickingWriter;"] # [doc = " impl Write for PanickingWriter {"] # [doc = "   fn write(&mut self, buf: &[u8]) -> io::Result<usize> { panic!() }"] # [doc = "   fn flush(&mut self) -> io::Result<()> { panic!() }"] # [doc = " }"] # [doc = ""] # [doc = " let mut stream = BufWriter::new(PanickingWriter);"] # [doc = " write!(stream, \"some data\").unwrap();"] # [doc = " let result = catch_unwind(AssertUnwindSafe(|| {"] # [doc = "     stream.flush().unwrap()"] # [doc = " }));"] # [doc = " assert!(result.is_err());"] # [doc = " let (recovered_writer, buffered_data) = stream.into_parts();"] # [doc = " assert!(matches!(recovered_writer, PanickingWriter));"] # [doc = " assert_eq!(buffered_data.unwrap_err().into_inner(), b\"some data\");"] # [doc = " ```"] pub struct WriterPanicked { buf : Vec < u8 > , }
};
}
