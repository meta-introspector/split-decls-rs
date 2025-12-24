use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Write is implemented for `ThinVec<u8>` by appending to the vector.
/// The vector will grow as needed.
/// This implementation is identical to the one for `Vec<u8>`.
#[cfg(feature = "std")]
impl std::io::Write for ThinVec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
