use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
pub mod bufread {
    //! BufRead-based hashing.
    //!
    //! Separate `BufRead` trait implemented to allow for custom buffer size optimization.
    //!
    //! # Example
    //! ```rust
    //! use std::io::{Cursor, BufReader};
    //! use simd_adler32::bufread::adler32;
    //!
    //! let mut reader = Cursor::new(b"Hello there");
    //! let mut reader = BufReader::new(reader);
    //! let hash = adler32(&mut reader).unwrap();
    //!
    //! println!("{}", hash) // 800813569
    //! ```
    use crate::Adler32;
    use std::io::{BufRead, ErrorKind, Result};
    /// Compute Adler-32 hash on buf reader until EOF.
    ///
    /// # Example
    /// ```rust
    /// use std::io::{Cursor, BufReader};
    /// use simd_adler32::bufread::adler32;
    ///
    /// let mut reader = Cursor::new(b"Hello there");
    /// let mut reader = BufReader::new(reader);
    /// let hash = adler32(&mut reader).unwrap();
    ///
    /// println!("{}", hash) // 800813569
    /// ```
    pub fn adler32<R: BufRead>(reader: &mut R) -> Result<u32> {
        let mut hash = Adler32::new();
        loop {
            let consumed = match reader.fill_buf() {
                Ok(buf) => {
                    if buf.is_empty() {
                        return Ok(hash.finish());
                    }
                    hash.write(buf);
                    buf.len()
                }
                Err(err) => {
                    match err.kind() {
                        ErrorKind::Interrupted => continue,
                        ErrorKind::UnexpectedEof => return Ok(hash.finish()),
                        _ => return Err(err),
                    }
                }
            };
            reader.consume(consumed);
        }
    }
}
