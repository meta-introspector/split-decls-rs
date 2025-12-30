// Generated macro for generic_copy (function)
macro_rules! Depcrate_io_copygeneric_copy {
() => {
// Module: crate::io::copy
// Provides: {"generic_copy"}
// Dependencies: {}
# [doc = " The userspace read-write-loop implementation of `io::copy` that is used when"] # [doc = " OS-specific specializations for copy offloading are not available or not applicable."] pub (crate) fn generic_copy < R : ? Sized , W : ? Sized > (reader : & mut R , writer : & mut W) -> Result < u64 > where R : Read , W : Write , { let read_buf = BufferedReaderSpec :: buffer_size (reader) ; let write_buf = BufferedWriterSpec :: buffer_size (writer) ; if read_buf >= DEFAULT_BUF_SIZE && read_buf >= write_buf { return BufferedReaderSpec :: copy_to (reader , writer) ; } BufferedWriterSpec :: copy_from (writer , reader) }
};
}
