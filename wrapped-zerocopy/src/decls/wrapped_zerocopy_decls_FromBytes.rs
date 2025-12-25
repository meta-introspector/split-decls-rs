use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types for which any bit pattern is valid.
///
/// Any memory region of the appropriate length which contains initialized bytes
/// can be viewed as any `FromBytes` type with no runtime overhead. This is
/// useful for efficiently parsing bytes as structured data.
///
/// # Warning: Padding bytes
///
/// Note that, when a value is moved or copied, only the non-padding bytes of
/// that value are guaranteed to be preserved. It is unsound to assume that
/// values written to padding bytes are preserved after a move or copy. For
/// example, the following is unsound:
///
/// ```rust,no_run
/// use core::mem::{size_of, transmute};
/// use zerocopy::FromZeros;
/// # use zerocopy_derive::*;
///
/// // Assume `Foo` is a type with padding bytes.
/// #[derive(FromZeros, Default)]
/// struct Foo {
/// # /*
///     ...
/// # */
/// }
///
/// let mut foo: Foo = Foo::default();
/// FromZeros::zero(&mut foo);
/// // UNSOUND: Although `FromZeros::zero` writes zeros to all bytes of `foo`,
/// // those writes are not guaranteed to be preserved in padding bytes when
/// // `foo` is moved, so this may expose padding bytes as `u8`s.
/// let foo_bytes: [u8; size_of::<Foo>()] = unsafe { transmute(foo) };
/// ```
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(FromBytes)]`][derive]; e.g.:
///
/// ```
/// # use zerocopy_derive::{FromBytes, Immutable};
/// #[derive(FromBytes)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   V00, V01, V02, V03, V04, V05, V06, V07, V08, V09, V0A, V0B, V0C, V0D, V0E,
/// #   V0F, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V1A, V1B, V1C, V1D,
/// #   V1E, V1F, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V2A, V2B, V2C,
/// #   V2D, V2E, V2F, V30, V31, V32, V33, V34, V35, V36, V37, V38, V39, V3A, V3B,
/// #   V3C, V3D, V3E, V3F, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V4A,
/// #   V4B, V4C, V4D, V4E, V4F, V50, V51, V52, V53, V54, V55, V56, V57, V58, V59,
/// #   V5A, V5B, V5C, V5D, V5E, V5F, V60, V61, V62, V63, V64, V65, V66, V67, V68,
/// #   V69, V6A, V6B, V6C, V6D, V6E, V6F, V70, V71, V72, V73, V74, V75, V76, V77,
/// #   V78, V79, V7A, V7B, V7C, V7D, V7E, V7F, V80, V81, V82, V83, V84, V85, V86,
/// #   V87, V88, V89, V8A, V8B, V8C, V8D, V8E, V8F, V90, V91, V92, V93, V94, V95,
/// #   V96, V97, V98, V99, V9A, V9B, V9C, V9D, V9E, V9F, VA0, VA1, VA2, VA3, VA4,
/// #   VA5, VA6, VA7, VA8, VA9, VAA, VAB, VAC, VAD, VAE, VAF, VB0, VB1, VB2, VB3,
/// #   VB4, VB5, VB6, VB7, VB8, VB9, VBA, VBB, VBC, VBD, VBE, VBF, VC0, VC1, VC2,
/// #   VC3, VC4, VC5, VC6, VC7, VC8, VC9, VCA, VCB, VCC, VCD, VCE, VCF, VD0, VD1,
/// #   VD2, VD3, VD4, VD5, VD6, VD7, VD8, VD9, VDA, VDB, VDC, VDD, VDE, VDF, VE0,
/// #   VE1, VE2, VE3, VE4, VE5, VE6, VE7, VE8, VE9, VEA, VEB, VEC, VED, VEE, VEF,
/// #   VF0, VF1, VF2, VF3, VF4, VF5, VF6, VF7, VF8, VF9, VFA, VFB, VFC, VFD, VFE,
/// #   VFF,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromBytes, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `FromBytes`.
///
/// # Safety
///
/// *This section describes what is required in order for `T: FromBytes`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `FromBytes` manually, and you don't plan on writing unsafe code that
/// operates on `FromBytes` types, then you don't need to read this section.*
///
/// If `T: FromBytes`, then unsafe code may assume that it is sound to produce a
/// `T` whose bytes are initialized to any sequence of valid `u8`s (in other
/// words, any byte value which is not uninitialized). If a type is marked as
/// `FromBytes` which violates this contract, it may cause undefined behavior.
///
/// `#[derive(FromBytes)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::FromBytes",
    doc = "[derive-analysis]: zerocopy_derive::FromBytes#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.FromBytes.html"
    ),
    doc = concat!(
        "[derive-analysis]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.FromBytes.html#analysis"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(note = "Consider adding `#[derive(FromBytes)]` to `{Self}`")
)]
pub unsafe trait FromBytes: FromZeros {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
    /// Interprets the given `source` as a `&Self`.
    ///
    /// This method attempts to return a reference to `source` interpreted as a
    /// `Self`. If the length of `source` is not a [valid size of
    /// `Self`][valid-size], or if `source` is not appropriately aligned, this
    /// returns `Err`. If [`Self: Unaligned`][self-unaligned], you can
    /// [infallibly discard the alignment error][size-error-from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::ref_from_bytes(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     header: PacketHeader,
    ///     body: [u8],
    /// }
    ///
    /// // These bytes encode a `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11][..];
    ///
    /// let packet = Packet::ref_from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(packet.header.src_port, [0, 1]);
    /// assert_eq!(packet.header.dst_port, [2, 3]);
    /// assert_eq!(packet.header.length, [4, 5]);
    /// assert_eq!(packet.header.checksum, [6, 7]);
    /// assert_eq!(packet.body, [8, 9, 10, 11]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_bytes(source: &[u8]) -> Result<&Self, CastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        static_assert_dst_is_not_zst!(Self);
        match Ptr::from_ref(source).try_cast_into_no_leftover::<_, BecauseImmutable>(None) {
            Ok(ptr) => Ok(ptr.recall_validity().as_ref()),
            Err(err) => Err(err.map_src(|src| src.as_ref())),
        }
    }
    /// Interprets the prefix of the given `source` as a `&Self` without
    /// copying.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the leading bytes of `source`, then attempts to return
    /// both a reference to those bytes interpreted as a `Self`, and a reference
    /// to the remaining bytes. If there are insufficient bytes, or if `source`
    /// is not appropriately aligned, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][size-error-from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`ref_from_prefix_with_elems`], which does
    /// support such types. Attempting to use this method on such types results
    /// in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::ref_from_prefix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// [`ref_from_prefix_with_elems`]: FromBytes::ref_from_prefix_with_elems
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// #[derive(FromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     header: PacketHeader,
    ///     body: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14][..];
    ///
    /// let (packet, suffix) = Packet::ref_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.header.src_port, [0, 1]);
    /// assert_eq!(packet.header.dst_port, [2, 3]);
    /// assert_eq!(packet.header.length, [4, 5]);
    /// assert_eq!(packet.header.checksum, [6, 7]);
    /// assert_eq!(packet.body, [[8, 9], [10, 11], [12, 13]]);
    /// assert_eq!(suffix, &[14u8][..]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_prefix(source: &[u8]) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        static_assert_dst_is_not_zst!(Self);
        ref_from_prefix_suffix(source, None, CastType::Prefix)
    }
    /// Interprets the suffix of the given bytes as a `&Self`.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the trailing bytes of `source`, then attempts to return
    /// both a reference to those bytes interpreted as a `Self`, and a reference
    /// to the preceding bytes. If there are insufficient bytes, or if that
    /// suffix of `source` is not appropriately aligned, this returns `Err`. If
    /// [`Self: Unaligned`][self-unaligned], you can [infallibly discard the
    /// alignment error][size-error-from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`ref_from_suffix_with_elems`], which does
    /// support such types. Attempting to use this method on such types results
    /// in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::ref_from_suffix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// [`ref_from_suffix_with_elems`]: FromBytes::ref_from_suffix_with_elems
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct PacketTrailer {
    ///     frame_check_sequence: [u8; 4],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketTrailer`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, trailer) = PacketTrailer::ref_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(prefix, &[0, 1, 2, 3, 4, 5][..]);
    /// assert_eq!(trailer.frame_check_sequence, [6, 7, 8, 9]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_suffix(source: &[u8]) -> Result<(&[u8], &Self), CastError<&[u8], Self>>
    where
        Self: Immutable + KnownLayout,
    {
        static_assert_dst_is_not_zst!(Self);
        ref_from_prefix_suffix(source, None, CastType::Suffix).map(swap)
    }
    /// Interprets the given `source` as a `&mut Self`.
    ///
    /// This method attempts to return a reference to `source` interpreted as a
    /// `Self`. If the length of `source` is not a [valid size of
    /// `Self`][valid-size], or if `source` is not appropriately aligned, this
    /// returns `Err`. If [`Self: Unaligned`][self-unaligned], you can
    /// [infallibly discard the alignment error][size-error-from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`mut_from_prefix_with_elems`], which does
    /// support such types. Attempting to use this method on such types results
    /// in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::mut_from_bytes(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// [`mut_from_prefix_with_elems`]: FromBytes::mut_from_prefix_with_elems
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These bytes encode a `PacketHeader`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let header = PacketHeader::mut_from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    ///
    /// header.checksum = [0, 0];
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 0, 0]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_bytes(source: &mut [u8]) -> Result<&mut Self, CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout,
    {
        static_assert_dst_is_not_zst!(Self);
        match Ptr::from_mut(source).try_cast_into_no_leftover::<_, BecauseExclusive>(None) {
            Ok(ptr) => Ok(ptr.recall_validity::<_, (_, (_, _))>().as_mut()),
            Err(err) => Err(err.map_src(|src| src.as_mut())),
        }
    }
    /// Interprets the prefix of the given `source` as a `&mut Self` without
    /// copying.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the leading bytes of `source`, then attempts to return
    /// both a reference to those bytes interpreted as a `Self`, and a reference
    /// to the remaining bytes. If there are insufficient bytes, or if `source`
    /// is not appropriately aligned, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][size-error-from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. See [`mut_from_suffix_with_elems`], which does
    /// support such types. Attempting to use this method on such types results
    /// in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::mut_from_prefix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// [`mut_from_suffix_with_elems`]: FromBytes::mut_from_suffix_with_elems
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketHeader`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (header, body) = PacketHeader::mut_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    /// assert_eq!(body, &[8, 9][..]);
    ///
    /// header.checksum = [0, 0];
    /// body.fill(1);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 0, 0, 1, 1]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_prefix(
        source: &mut [u8],
    ) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout,
    {
        static_assert_dst_is_not_zst!(Self);
        mut_from_prefix_suffix(source, None, CastType::Prefix)
    }
    /// Interprets the suffix of the given `source` as a `&mut Self` without
    /// copying.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the trailing bytes of `source`, then attempts to return
    /// both a reference to those bytes interpreted as a `Self`, and a reference
    /// to the preceding bytes. If there are insufficient bytes, or if that
    /// suffix of `source` is not appropriately aligned, this returns `Err`. If
    /// [`Self: Unaligned`][self-unaligned], you can [infallibly discard the
    /// alignment error][size-error-from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Compile-Time Assertions
    ///
    /// This method cannot yet be used on unsized types whose dynamically-sized
    /// component is zero-sized. Attempting to use this method on such types
    /// results in a compile-time assertion error; e.g.:
    ///
    /// ```compile_fail,E0080
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::mut_from_suffix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct PacketTrailer {
    ///     frame_check_sequence: [u8; 4],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketTrailer`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, trailer) = PacketTrailer::mut_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(prefix, &[0u8, 1, 2, 3, 4, 5][..]);
    /// assert_eq!(trailer.frame_check_sequence, [6, 7, 8, 9]);
    ///
    /// prefix.fill(0);
    /// trailer.frame_check_sequence.fill(1);
    ///
    /// assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 1, 1, 1, 1]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_suffix(
        source: &mut [u8],
    ) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout,
    {
        static_assert_dst_is_not_zst!(Self);
        mut_from_prefix_suffix(source, None, CastType::Suffix).map(swap)
    }
    /// Interprets the given `source` as a `&Self` with a DST length equal to
    /// `count`.
    ///
    /// This method attempts to return a reference to `source` interpreted as a
    /// `Self` with `count` trailing elements. If the length of `source` is not
    /// equal to the size of `Self` with `count` elements, or if `source` is not
    /// appropriately aligned, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][size-error-from].
    ///
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let pixels = <[Pixel]>::ref_from_bytes_with_elems(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`ref_from_bytes`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &[85, 85][..];
    /// let zsty = ZSTy::ref_from_bytes_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`ref_from_bytes`]: FromBytes::ref_from_bytes
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_bytes_with_elems(
        source: &[u8],
        count: usize,
    ) -> Result<&Self, CastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        let source = Ptr::from_ref(source);
        let maybe_slf = source.try_cast_into_no_leftover::<_, BecauseImmutable>(Some(count));
        match maybe_slf {
            Ok(slf) => Ok(slf.recall_validity().as_ref()),
            Err(err) => Err(err.map_src(|s| s.as_ref())),
        }
    }
    /// Interprets the prefix of the given `source` as a DST `&Self` with length
    /// equal to `count`.
    ///
    /// This method attempts to return a reference to the prefix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the remaining bytes. If there are insufficient bytes, or if `source`
    /// is not appropriately aligned, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][size-error-from].
    ///
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (pixels, suffix) = <[Pixel]>::ref_from_prefix_with_elems(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// assert_eq!(suffix, &[8, 9]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`ref_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &[85, 85][..];
    /// let (zsty, _) = ZSTy::ref_from_prefix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`ref_from_prefix`]: FromBytes::ref_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_prefix_with_elems(
        source: &[u8],
        count: usize,
    ) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        ref_from_prefix_suffix(source, Some(count), CastType::Prefix)
    }
    /// Interprets the suffix of the given `source` as a DST `&Self` with length
    /// equal to `count`.
    ///
    /// This method attempts to return a reference to the suffix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the preceding bytes. If there are insufficient bytes, or if that
    /// suffix of `source` is not appropriately aligned, this returns `Err`. If
    /// [`Self: Unaligned`][self-unaligned], you can [infallibly discard the
    /// alignment error][size-error-from].
    ///
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, pixels) = <[Pixel]>::ref_from_suffix_with_elems(bytes, 2).unwrap();
    ///
    /// assert_eq!(prefix, &[0, 1]);
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 2, g: 3, b: 4, a: 5 },
    ///     Pixel { r: 6, g: 7, b: 8, a: 9 },
    /// ]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`ref_from_suffix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &[85, 85][..];
    /// let (_, zsty) = ZSTy::ref_from_suffix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`ref_from_suffix`]: FromBytes::ref_from_suffix
    #[must_use = "has no side effects"]
    #[inline]
    fn ref_from_suffix_with_elems(
        source: &[u8],
        count: usize,
    ) -> Result<(&[u8], &Self), CastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        ref_from_prefix_suffix(source, Some(count), CastType::Suffix).map(swap)
    }
    /// Interprets the given `source` as a `&mut Self` with a DST length equal
    /// to `count`.
    ///
    /// This method attempts to return a reference to `source` interpreted as a
    /// `Self` with `count` trailing elements. If the length of `source` is not
    /// equal to the size of `Self` with `count` elements, or if `source` is not
    /// appropriately aligned, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][size-error-from].
    ///
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let pixels = <[Pixel]>::mut_from_bytes_with_elems(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// pixels[1] = Pixel { r: 0, g: 0, b: 0, a: 0 };
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 0, 0, 0, 0]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`mut_from`] which
    /// do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &mut [85, 85][..];
    /// let zsty = ZSTy::mut_from_bytes_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`mut_from`]: FromBytes::mut_from
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_bytes_with_elems(
        source: &mut [u8],
        count: usize,
    ) -> Result<&mut Self, CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout<PointerMetadata = usize> + Immutable,
    {
        let source = Ptr::from_mut(source);
        let maybe_slf = source.try_cast_into_no_leftover::<_, BecauseImmutable>(Some(count));
        match maybe_slf {
            Ok(slf) => Ok(slf
                .recall_validity::<_, (_, (_, (BecauseExclusive, BecauseExclusive)))>()
                .as_mut()),
            Err(err) => Err(err.map_src(|s| s.as_mut())),
        }
    }
    /// Interprets the prefix of the given `source` as a `&mut Self` with DST
    /// length equal to `count`.
    ///
    /// This method attempts to return a reference to the prefix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the preceding bytes. If there are insufficient bytes, or if `source`
    /// is not appropriately aligned, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][size-error-from].
    ///
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (pixels, suffix) = <[Pixel]>::mut_from_prefix_with_elems(bytes, 2).unwrap();
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 0, g: 1, b: 2, a: 3 },
    ///     Pixel { r: 4, g: 5, b: 6, a: 7 },
    /// ]);
    ///
    /// assert_eq!(suffix, &[8, 9]);
    ///
    /// pixels[1] = Pixel { r: 0, g: 0, b: 0, a: 0 };
    /// suffix.fill(1);
    ///
    /// assert_eq!(bytes, [0, 1, 2, 3, 0, 0, 0, 0, 1, 1]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`mut_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &mut [85, 85][..];
    /// let (zsty, _) = ZSTy::mut_from_prefix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`mut_from_prefix`]: FromBytes::mut_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_prefix_with_elems(
        source: &mut [u8],
        count: usize,
    ) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout<PointerMetadata = usize>,
    {
        mut_from_prefix_suffix(source, Some(count), CastType::Prefix)
    }
    /// Interprets the suffix of the given `source` as a `&mut Self` with DST
    /// length equal to `count`.
    ///
    /// This method attempts to return a reference to the suffix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the remaining bytes. If there are insufficient bytes, or if that
    /// suffix of `source` is not appropriately aligned, this returns `Err`. If
    /// [`Self: Unaligned`][self-unaligned], you can [infallibly discard the
    /// alignment error][size-error-from].
    ///
    /// [self-unaligned]: Unaligned
    /// [size-error-from]: error/struct.SizeError.html#method.from-1
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// # #[derive(Debug, PartialEq, Eq)]
    /// #[derive(FromBytes, IntoBytes, Immutable)]
    /// #[repr(C)]
    /// struct Pixel {
    ///     r: u8,
    ///     g: u8,
    ///     b: u8,
    ///     a: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode two `Pixel`s.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, pixels) = <[Pixel]>::mut_from_suffix_with_elems(bytes, 2).unwrap();
    ///
    /// assert_eq!(prefix, &[0, 1]);
    ///
    /// assert_eq!(pixels, &[
    ///     Pixel { r: 2, g: 3, b: 4, a: 5 },
    ///     Pixel { r: 6, g: 7, b: 8, a: 9 },
    /// ]);
    ///
    /// prefix.fill(9);
    /// pixels[1] = Pixel { r: 0, g: 0, b: 0, a: 0 };
    ///
    /// assert_eq!(bytes, [9, 9, 2, 3, 4, 5, 0, 0, 0, 0]);
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`mut_from_suffix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = &mut [85, 85][..];
    /// let (_, zsty) = ZSTy::mut_from_suffix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`mut_from_suffix`]: FromBytes::mut_from_suffix
    #[must_use = "has no side effects"]
    #[inline]
    fn mut_from_suffix_with_elems(
        source: &mut [u8],
        count: usize,
    ) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>
    where
        Self: IntoBytes + KnownLayout<PointerMetadata = usize>,
    {
        mut_from_prefix_suffix(source, Some(count), CastType::Suffix).map(swap)
    }
    /// Reads a copy of `Self` from the given `source`.
    ///
    /// If `source.len() != size_of::<Self>()`, `read_from_bytes` returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These bytes encode a `PacketHeader`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let header = PacketHeader::read_from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn read_from_bytes(source: &[u8]) -> Result<Self, SizeError<&[u8], Self>>
    where
        Self: Sized,
    {
        match Ref::<_, Unalign<Self>>::sized_from(source) {
            Ok(r) => Ok(Ref::read(&r).into_inner()),
            Err(CastError::Size(e)) => Err(e.with_dst()),
            Err(CastError::Alignment(_)) => unsafe { core::hint::unreachable_unchecked() },
            Err(CastError::Validity(i)) => match i {},
        }
    }
    /// Reads a copy of `Self` from the prefix of the given `source`.
    ///
    /// This attempts to read a `Self` from the first `size_of::<Self>()` bytes
    /// of `source`, returning that `Self` and any remaining bytes. If
    /// `source.len() < size_of::<Self>()`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketHeader`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (header, body) = PacketHeader::read_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(header.src_port, [0, 1]);
    /// assert_eq!(header.dst_port, [2, 3]);
    /// assert_eq!(header.length, [4, 5]);
    /// assert_eq!(header.checksum, [6, 7]);
    /// assert_eq!(body, [8, 9]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn read_from_prefix(source: &[u8]) -> Result<(Self, &[u8]), SizeError<&[u8], Self>>
    where
        Self: Sized,
    {
        match Ref::<_, Unalign<Self>>::sized_from_prefix(source) {
            Ok((r, suffix)) => Ok((Ref::read(&r).into_inner(), suffix)),
            Err(CastError::Size(e)) => Err(e.with_dst()),
            Err(CastError::Alignment(_)) => unsafe { core::hint::unreachable_unchecked() },
            Err(CastError::Validity(i)) => match i {},
        }
    }
    /// Reads a copy of `Self` from the suffix of the given `source`.
    ///
    /// This attempts to read a `Self` from the last `size_of::<Self>()` bytes
    /// of `source`, returning that `Self` and any preceding bytes. If
    /// `source.len() < size_of::<Self>()`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::FromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct PacketTrailer {
    ///     frame_check_sequence: [u8; 4],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `PacketTrailer`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..];
    ///
    /// let (prefix, trailer) = PacketTrailer::read_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(prefix, [0, 1, 2, 3, 4, 5]);
    /// assert_eq!(trailer.frame_check_sequence, [6, 7, 8, 9]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn read_from_suffix(source: &[u8]) -> Result<(&[u8], Self), SizeError<&[u8], Self>>
    where
        Self: Sized,
    {
        match Ref::<_, Unalign<Self>>::sized_from_suffix(source) {
            Ok((prefix, r)) => Ok((prefix, Ref::read(&r).into_inner())),
            Err(CastError::Size(e)) => Err(e.with_dst()),
            Err(CastError::Alignment(_)) => unsafe { core::hint::unreachable_unchecked() },
            Err(CastError::Validity(i)) => match i {},
        }
    }
    /// Reads a copy of `self` from an `io::Read`.
    ///
    /// This is useful for interfacing with operating system byte sinks (files,
    /// sockets, etc.).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use zerocopy::{byteorder::big_endian::*, FromBytes};
    /// use std::fs::File;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(FromBytes)]
    /// #[repr(C)]
    /// struct BitmapFileHeader {
    ///     signature: [u8; 2],
    ///     size: U32,
    ///     reserved: U64,
    ///     offset: U64,
    /// }
    ///
    /// let mut file = File::open("image.bin").unwrap();
    /// let header = BitmapFileHeader::read_from_io(&mut file).unwrap();
    /// ```
    #[cfg(feature = "std")]
    #[inline(always)]
    fn read_from_io<R>(mut src: R) -> io::Result<Self>
    where
        Self: Sized,
        R: io::Read,
    {
        let mut buf = CoreMaybeUninit::<Self>::uninit();
        buf.zero();
        let ptr = Ptr::from_mut(&mut buf);
        let ptr = unsafe { ptr.assume_validity::<invariant::Initialized>() };
        let ptr = ptr.as_bytes::<BecauseExclusive>();
        src.read_exact(ptr.as_mut())?;
        Ok(unsafe { buf.assume_init() })
    }
    #[deprecated(since = "0.8.0", note = "renamed to `FromBytes::ref_from_bytes`")]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn ref_from(source: &[u8]) -> Option<&Self>
    where
        Self: KnownLayout + Immutable,
    {
        Self::ref_from_bytes(source).ok()
    }
    #[deprecated(since = "0.8.0", note = "renamed to `FromBytes::mut_from_bytes`")]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn mut_from(source: &mut [u8]) -> Option<&mut Self>
    where
        Self: KnownLayout + IntoBytes,
    {
        Self::mut_from_bytes(source).ok()
    }
    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::ref_from_prefix_with_elems`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn slice_from_prefix(source: &[u8], count: usize) -> Option<(&[Self], &[u8])>
    where
        Self: Sized + Immutable,
    {
        <[Self]>::ref_from_prefix_with_elems(source, count).ok()
    }
    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::ref_from_suffix_with_elems`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn slice_from_suffix(source: &[u8], count: usize) -> Option<(&[u8], &[Self])>
    where
        Self: Sized + Immutable,
    {
        <[Self]>::ref_from_suffix_with_elems(source, count).ok()
    }
    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::mut_from_prefix_with_elems`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn mut_slice_from_prefix(source: &mut [u8], count: usize) -> Option<(&mut [Self], &mut [u8])>
    where
        Self: Sized + IntoBytes,
    {
        <[Self]>::mut_from_prefix_with_elems(source, count).ok()
    }
    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromBytes::mut_from_suffix_with_elems`"
    )]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn mut_slice_from_suffix(source: &mut [u8], count: usize) -> Option<(&mut [u8], &mut [Self])>
    where
        Self: Sized + IntoBytes,
    {
        <[Self]>::mut_from_suffix_with_elems(source, count).ok()
    }
    #[deprecated(since = "0.8.0", note = "renamed to `FromBytes::read_from_bytes`")]
    #[doc(hidden)]
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn read_from(source: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        Self::read_from_bytes(source).ok()
    }
}
