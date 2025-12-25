use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types that can be converted to an immutable slice of initialized bytes.
///
/// Any `IntoBytes` type can be converted to a slice of initialized bytes of the
/// same size. This is useful for efficiently serializing structured data as raw
/// bytes.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(IntoBytes)]`][derive]; e.g.:
///
/// ```
/// # use zerocopy_derive::IntoBytes;
/// #[derive(IntoBytes)]
/// #[repr(C)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(IntoBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `IntoBytes`. See the [derive
/// documentation][derive] for guidance on how to interpret error messages
/// produced by the derive's analysis.
///
/// # Safety
///
/// *This section describes what is required in order for `T: IntoBytes`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `IntoBytes` manually, and you don't plan on writing unsafe code that
/// operates on `IntoBytes` types, then you don't need to read this section.*
///
/// If `T: IntoBytes`, then unsafe code may assume that it is sound to treat any
/// `t: T` as an immutable `[u8]` of length `size_of_val(t)`. If a type is
/// marked as `IntoBytes` which violates this contract, it may cause undefined
/// behavior.
///
/// `#[derive(IntoBytes)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::IntoBytes",
    doc = "[derive-analysis]: zerocopy_derive::IntoBytes#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.IntoBytes.html"
    ),
    doc = concat!(
        "[derive-analysis]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.IntoBytes.html#analysis"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(
        note = "Consider adding `#[derive(IntoBytes)]` to `{Self}`"
    )
)]
pub unsafe trait IntoBytes {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
    /// Gets the bytes of this value.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let bytes = header.as_bytes();
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn as_bytes(&self) -> &[u8]
    where
        Self: Immutable,
    {
        let len = mem::size_of_val(self);
        let slf: *const Self = self;
        unsafe { slice::from_raw_parts(slf.cast::<u8>(), len) }
    }
    /// Gets the bytes of this value mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Eq, PartialEq, Debug)]
    /// #[derive(FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let mut header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let bytes = header.as_mut_bytes();
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    ///
    /// bytes.reverse();
    ///
    /// assert_eq!(header, PacketHeader {
    ///     src_port: [7, 6],
    ///     dst_port: [5, 4],
    ///     length: [3, 2],
    ///     checksum: [1, 0],
    /// });
    /// ```
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn as_mut_bytes(&mut self) -> &mut [u8]
    where
        Self: FromBytes,
    {
        let len = mem::size_of_val(self);
        let slf: *mut Self = self;
        unsafe { slice::from_raw_parts_mut(slf.cast::<u8>(), len) }
    }
    /// Writes a copy of `self` to `dst`.
    ///
    /// If `dst.len() != size_of_val(self)`, `write_to` returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0];
    ///
    /// header.write_to(&mut bytes[..]);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    /// ```
    ///
    /// If too many or too few target bytes are provided, `write_to` returns
    /// `Err` and leaves the target bytes unmodified:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    /// # let header = u128::MAX;
    /// let mut excessive_bytes = &mut [0u8; 128][..];
    ///
    /// let write_result = header.write_to(excessive_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(excessive_bytes, [0u8; 128]);
    /// ```
    #[must_use = "callers should check the return value to see if the operation succeeded"]
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn write_to(&self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
    where
        Self: Immutable,
    {
        let src = self.as_bytes();
        if dst.len() == src.len() {
            unsafe { util::copy_unchecked(src, dst) }
            Ok(())
        } else {
            Err(SizeError::new(self))
        }
    }
    /// Writes a copy of `self` to the prefix of `dst`.
    ///
    /// `write_to_prefix` writes `self` to the first `size_of_val(self)` bytes
    /// of `dst`. If `dst.len() < size_of_val(self)`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    ///
    /// header.write_to_prefix(&mut bytes[..]);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7, 0, 0]);
    /// ```
    ///
    /// If insufficient target bytes are provided, `write_to_prefix` returns
    /// `Err` and leaves the target bytes unmodified:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    /// # let header = u128::MAX;
    /// let mut insufficient_bytes = &mut [0, 0][..];
    ///
    /// let write_result = header.write_to_suffix(insufficient_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(insufficient_bytes, [0, 0]);
    /// ```
    #[must_use = "callers should check the return value to see if the operation succeeded"]
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn write_to_prefix(&self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
    where
        Self: Immutable,
    {
        let src = self.as_bytes();
        match dst.get_mut(..src.len()) {
            Some(dst) => {
                unsafe { util::copy_unchecked(src, dst) }
                Ok(())
            }
            None => Err(SizeError::new(self)),
        }
    }
    /// Writes a copy of `self` to the suffix of `dst`.
    ///
    /// `write_to_suffix` writes `self` to the last `size_of_val(self)` bytes of
    /// `dst`. If `dst.len() < size_of_val(self)`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::IntoBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header = PacketHeader {
    ///     src_port: [0, 1],
    ///     dst_port: [2, 3],
    ///     length: [4, 5],
    ///     checksum: [6, 7],
    /// };
    ///
    /// let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    ///
    /// header.write_to_suffix(&mut bytes[..]);
    ///
    /// assert_eq!(bytes, [0, 0, 0, 1, 2, 3, 4, 5, 6, 7]);
    ///
    /// let mut insufficient_bytes = &mut [0, 0][..];
    ///
    /// let write_result = header.write_to_suffix(insufficient_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(insufficient_bytes, [0, 0]);
    /// ```
    ///
    /// If insufficient target bytes are provided, `write_to_suffix` returns
    /// `Err` and leaves the target bytes unmodified:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    /// # let header = u128::MAX;
    /// let mut insufficient_bytes = &mut [0, 0][..];
    ///
    /// let write_result = header.write_to_suffix(insufficient_bytes);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(insufficient_bytes, [0, 0]);
    /// ```
    #[must_use = "callers should check the return value to see if the operation succeeded"]
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn write_to_suffix(&self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>
    where
        Self: Immutable,
    {
        let src = self.as_bytes();
        let start = if let Some(start) = dst.len().checked_sub(src.len()) {
            start
        } else {
            return Err(SizeError::new(self));
        };
        let dst = if let Some(dst) = dst.get_mut(start..) {
            dst
        } else {
            return Err(SizeError::new(self));
        };
        unsafe {
            util::copy_unchecked(src, dst);
        }
        Ok(())
    }
    /// Writes a copy of `self` to an `io::Write`.
    ///
    /// This is a shorthand for `dst.write_all(self.as_bytes())`, and is useful
    /// for interfacing with operating system byte sinks (files, sockets, etc.).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use zerocopy::{byteorder::big_endian::U16, FromBytes, IntoBytes};
    /// use std::fs::File;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct GrayscaleImage {
    ///     height: U16,
    ///     width: U16,
    ///     pixels: [U16],
    /// }
    ///
    /// let image = GrayscaleImage::ref_from_bytes(&[0, 0, 0, 0][..]).unwrap();
    /// let mut file = File::create("image.bin").unwrap();
    /// image.write_to_io(&mut file).unwrap();
    /// ```
    ///
    /// If the write fails, `write_to_io` returns `Err` and a partial write may
    /// have occurred; e.g.:
    ///
    /// ```
    /// # use zerocopy::IntoBytes;
    ///
    /// let src = u128::MAX;
    /// let mut dst = [0u8; 2];
    ///
    /// let write_result = src.write_to_io(&mut dst[..]);
    ///
    /// assert!(write_result.is_err());
    /// assert_eq!(dst, [255, 255]);
    /// ```
    #[cfg(feature = "std")]
    #[inline(always)]
    fn write_to_io<W>(&self, mut dst: W) -> io::Result<()>
    where
        Self: Immutable,
        W: io::Write,
    {
        dst.write_all(self.as_bytes())
    }
    #[deprecated(
        since = "0.8.0",
        note = "`IntoBytes::as_bytes_mut` was renamed to `as_mut_bytes`"
    )]
    #[doc(hidden)]
    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8]
    where
        Self: FromBytes,
    {
        self.as_mut_bytes()
    }
}
