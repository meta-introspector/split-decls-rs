// Generated macro for impl_893 (impl)
macro_rules! Depcrate_io_buffered_bufreaderimpl_893 {
() => {
// Module: crate::io::buffered::bufreader
// Provides: {"impl_893"}
// Dependencies: {}
impl < R : Read > BufReader < R > { # [doc = " Creates a new `BufReader<R>` with a default buffer capacity. The default is currently 8 KiB,"] # [doc = " but may change in the future."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let reader = BufReader::new(f);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn new (inner : R) -> BufReader < R > { BufReader :: with_capacity (DEFAULT_BUF_SIZE , inner) } pub (crate) fn try_new_buffer () -> io :: Result < Buffer > { Buffer :: try_with_capacity (DEFAULT_BUF_SIZE) } pub (crate) fn with_buffer (inner : R , buf : Buffer) -> Self { Self { inner , buf } } # [doc = " Creates a new `BufReader<R>` with the specified buffer capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Creating a buffer with ten bytes of capacity:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::BufReader;"] # [doc = " use std::fs::File;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let f = File::open(\"log.txt\")?;"] # [doc = "     let reader = BufReader::with_capacity(10, f);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn with_capacity (capacity : usize , inner : R) -> BufReader < R > { BufReader { inner , buf : Buffer :: with_capacity (capacity) } } }
};
}
