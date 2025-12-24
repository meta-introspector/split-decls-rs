use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
pub mod read {
    //! Reader-based hashing.
    //!
    //! # Example
    //! ```rust
    //! use std::io::Cursor;
    //! use simd_adler32::read::adler32;
    //!
    //! let mut reader = Cursor::new(b"Hello there");
    //! let hash = adler32(&mut reader).unwrap();
    //!
    //! println!("{}", hash) // 800813569
    //! ```
    use crate::Adler32;
    use std::io::{Read, Result};
    /// Compute Adler-32 hash on reader until EOF.
    ///
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use simd_adler32::read::adler32;
    ///
    /// let mut reader = Cursor::new(b"Hello there");
    /// let hash = adler32(&mut reader).unwrap();
    ///
    /// println!("{}", hash) // 800813569
    /// ```
    pub fn adler32<R: Read>(reader: &mut R) -> Result<u32> {
        let mut hash = Adler32::new();
        let mut buf = [0; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => return Ok(hash.finish()),
                Ok(n) => {
                    hash.write(&buf[..n]);
                }
                Err(err) => return Err(err),
            }
        }
    }
}
