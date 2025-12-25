use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types for which some bit patterns are valid.
///
/// A memory region of the appropriate length which contains initialized bytes
/// can be viewed as a `TryFromBytes` type so long as the runtime value of those
/// bytes corresponds to a [*valid instance*] of that type. For example,
/// [`bool`] is `TryFromBytes`, so zerocopy can transmute a [`u8`] into a
/// [`bool`] so long as it first checks that the value of the [`u8`] is `0` or
/// `1`.
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(TryFromBytes)]`][derive]; e.g.:
///
/// ```
/// # use zerocopy_derive::{TryFromBytes, Immutable};
/// #[derive(TryFromBytes)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(TryFromBytes)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   V00,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(TryFromBytes, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive ensures that the runtime check of whether bytes correspond to a
/// valid instance is sound. You **must** implement this trait via the derive.
///
/// # What is a "valid instance"?
///
/// In Rust, each type has *bit validity*, which refers to the set of bit
/// patterns which may appear in an instance of that type. It is impossible for
/// safe Rust code to produce values which violate bit validity (ie, values
/// outside of the "valid" set of bit patterns). If `unsafe` code produces an
/// invalid value, this is considered [undefined behavior].
///
/// Rust's bit validity rules are currently being decided, which means that some
/// types have three classes of bit patterns: those which are definitely valid,
/// and whose validity is documented in the language; those which may or may not
/// be considered valid at some point in the future; and those which are
/// definitely invalid.
///
/// Zerocopy takes a conservative approach, and only considers a bit pattern to
/// be valid if its validity is a documented guarantee provided by the
/// language.
///
/// For most use cases, Rust's current guarantees align with programmers'
/// intuitions about what ought to be valid. As a result, zerocopy's
/// conservatism should not affect most users.
///
/// If you are negatively affected by lack of support for a particular type,
/// we encourage you to let us know by [filing an issue][github-repo].
///
/// # `TryFromBytes` is not symmetrical with [`IntoBytes`]
///
/// There are some types which implement both `TryFromBytes` and [`IntoBytes`],
/// but for which `TryFromBytes` is not guaranteed to accept all byte sequences
/// produced by `IntoBytes`. In other words, for some `T: TryFromBytes +
/// IntoBytes`, there exist values of `t: T` such that
/// `TryFromBytes::try_ref_from_bytes(t.as_bytes()) == None`. Code should not
/// generally assume that values produced by `IntoBytes` will necessarily be
/// accepted as valid by `TryFromBytes`.
///
/// # Safety
///
/// On its own, `T: TryFromBytes` does not make any guarantees about the layout
/// or representation of `T`. It merely provides the ability to perform a
/// validity check at runtime via methods like [`try_ref_from_bytes`].
///
/// You must not rely on the `#[doc(hidden)]` internals of `TryFromBytes`.
/// Future releases of zerocopy may make backwards-breaking changes to these
/// items, including changes that only affect soundness, which may cause code
/// which uses those items to silently become unsound.
///
/// [undefined behavior]: https://raphlinus.github.io/programming/rust/2018/08/17/undefined-behavior.html
/// [github-repo]: https://github.com/google/zerocopy
/// [`try_ref_from_bytes`]: TryFromBytes::try_ref_from_bytes
/// [*valid instance*]: #what-is-a-valid-instance
#[cfg_attr(feature = "derive", doc = "[derive]: zerocopy_derive::TryFromBytes")]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.TryFromBytes.html"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(note = "Consider adding `#[derive(TryFromBytes)]` to `{Self}`")
)]
pub unsafe trait TryFromBytes {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
    /// Does a given memory range contain a valid instance of `Self`?
    ///
    /// # Safety
    ///
    /// Unsafe code may assume that, if `is_bit_valid(candidate)` returns true,
    /// `*candidate` contains a valid `Self`.
    ///
    /// # Panics
    ///
    /// `is_bit_valid` may panic. Callers are responsible for ensuring that any
    /// `unsafe` code remains sound even in the face of `is_bit_valid`
    /// panicking. (We support user-defined validation routines; so long as
    /// these routines are not required to be `unsafe`, there is no way to
    /// ensure that these do not generate panics.)
    ///
    /// Besides user-defined validation routines panicking, `is_bit_valid` will
    /// either panic or fail to compile if called on a pointer with [`Shared`]
    /// aliasing when `Self: !Immutable`.
    ///
    /// [`UnsafeCell`]: core::cell::UnsafeCell
    /// [`Shared`]: invariant::Shared
    #[doc(hidden)]
    fn is_bit_valid<A: invariant::Reference>(candidate: Maybe<'_, Self, A>) -> bool;
    /// Attempts to interpret the given `source` as a `&Self`.
    ///
    /// If the bytes of `source` are a valid instance of `Self`, this method
    /// returns a reference to those bytes interpreted as a `Self`. If the
    /// length of `source` is not a [valid size of `Self`][valid-size], or if
    /// `source` is not appropriately aligned, or if `source` is not a valid
    /// instance of `Self`, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][ConvertError::from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
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
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::try_ref_from_bytes(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the byte sequence `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &[0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5][..];
    ///
    /// let packet = Packet::try_ref_from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0x10, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5][..];
    /// assert!(Packet::try_ref_from_bytes(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_bytes(source: &[u8]) -> Result<&Self, TryCastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        static_assert_dst_is_not_zst!(Self);
        match Ptr::from_ref(source).try_cast_into_no_leftover::<Self, BecauseImmutable>(None) {
            Ok(source) => match source.try_into_valid() {
                Ok(valid) => Ok(valid.as_ref()),
                Err(e) => Err(e
                    .map_src(|src| src.as_bytes::<BecauseImmutable>().as_ref())
                    .into()),
            },
            Err(e) => Err(e.map_src(Ptr::as_ref).into()),
        }
    }
    /// Attempts to interpret the prefix of the given `source` as a `&Self`.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the leading bytes of `source`. If that prefix is a valid
    /// instance of `Self`, this method returns a reference to those bytes
    /// interpreted as `Self`, and a reference to the remaining bytes. If there
    /// are insufficient bytes, or if `source` is not appropriately aligned, or
    /// if those bytes are not a valid instance of `Self`, this returns `Err`.
    /// If [`Self: Unaligned`][self-unaligned], you can [infallibly discard the
    /// alignment error][ConvertError::from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
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
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::try_ref_from_prefix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    ///
    /// let (packet, suffix) = Packet::try_ref_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    /// assert_eq!(suffix, &[6u8][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0x10, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_ref_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_prefix(source: &[u8]) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        static_assert_dst_is_not_zst!(Self);
        try_ref_from_prefix_suffix(source, CastType::Prefix, None)
    }
    /// Attempts to interpret the suffix of the given `source` as a `&Self`.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the trailing bytes of `source`. If that suffix is a
    /// valid instance of `Self`, this method returns a reference to those bytes
    /// interpreted as `Self`, and a reference to the preceding bytes. If there
    /// are insufficient bytes, or if the suffix of `source` would not be
    /// appropriately aligned, or if the suffix is not a valid instance of
    /// `Self`, this returns `Err`. If [`Self: Unaligned`][self-unaligned], you
    /// can [infallibly discard the alignment error][ConvertError::from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
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
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let _ = ZSTy::try_ref_from_suffix(0u16.as_bytes()); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0, 0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let (prefix, packet) = Packet::try_ref_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(prefix, &[0u8][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 77, 240, 0xC0, 0x10][..];
    /// assert!(Packet::try_ref_from_suffix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_suffix(source: &[u8]) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>
    where
        Self: KnownLayout + Immutable,
    {
        static_assert_dst_is_not_zst!(Self);
        try_ref_from_prefix_suffix(source, CastType::Suffix, None).map(swap)
    }
    /// Attempts to interpret the given `source` as a `&mut Self` without
    /// copying.
    ///
    /// If the bytes of `source` are a valid instance of `Self`, this method
    /// returns a reference to those bytes interpreted as a `Self`. If the
    /// length of `source` is not a [valid size of `Self`][valid-size], or if
    /// `source` is not appropriately aligned, or if `source` is not a valid
    /// instance of `Self`, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][ConvertError::from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
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
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::try_mut_from_bytes(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &mut [0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5][..];
    ///
    /// let packet = Packet::try_mut_from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    ///
    /// packet.temperature = 111;
    ///
    /// assert_eq!(bytes, [0xC0, 0xC0, 240, 111, 0, 1, 2, 3, 4, 5]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0x10, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_mut_from_bytes(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_bytes(bytes: &mut [u8]) -> Result<&mut Self, TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout + IntoBytes,
    {
        static_assert_dst_is_not_zst!(Self);
        match Ptr::from_mut(bytes).try_cast_into_no_leftover::<Self, BecauseExclusive>(None) {
            Ok(source) => match source.try_into_valid() {
                Ok(source) => Ok(source.as_mut()),
                Err(e) => Err(e
                    .map_src(|src| src.as_bytes::<BecauseExclusive>().as_mut())
                    .into()),
            },
            Err(e) => Err(e.map_src(Ptr::as_mut).into()),
        }
    }
    /// Attempts to interpret the prefix of the given `source` as a `&mut
    /// Self`.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the leading bytes of `source`. If that prefix is a valid
    /// instance of `Self`, this method returns a reference to those bytes
    /// interpreted as `Self`, and a reference to the remaining bytes. If there
    /// are insufficient bytes, or if `source` is not appropriately aligned, or
    /// if the bytes are not a valid instance of `Self`, this returns `Err`. If
    /// [`Self: Unaligned`][self-unaligned], you can [infallibly discard the
    /// alignment error][ConvertError::from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
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
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: [u8; 2],
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::try_mut_from_prefix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &mut [0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    ///
    /// let (packet, suffix) = Packet::try_mut_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[0, 1], [2, 3], [4, 5]]);
    /// assert_eq!(suffix, &[6u8][..]);
    ///
    /// packet.temperature = 111;
    /// suffix[0] = 222;
    ///
    /// assert_eq!(bytes, [0xC0, 0xC0, 240, 111, 0, 1, 2, 3, 4, 5, 222]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0x10, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_mut_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_prefix(
        source: &mut [u8],
    ) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout + IntoBytes,
    {
        static_assert_dst_is_not_zst!(Self);
        try_mut_from_prefix_suffix(source, CastType::Prefix, None)
    }
    /// Attempts to interpret the suffix of the given `source` as a `&mut
    /// Self`.
    ///
    /// This method computes the [largest possible size of `Self`][valid-size]
    /// that can fit in the trailing bytes of `source`. If that suffix is a
    /// valid instance of `Self`, this method returns a reference to those bytes
    /// interpreted as `Self`, and a reference to the preceding bytes. If there
    /// are insufficient bytes, or if the suffix of `source` would not be
    /// appropriately aligned, or if the suffix is not a valid instance of
    /// `Self`, this returns `Err`. If [`Self: Unaligned`][self-unaligned], you
    /// can [infallibly discard the alignment error][ConvertError::from].
    ///
    /// `Self` may be a sized type, a slice, or a [slice DST][slice-dst].
    ///
    /// [valid-size]: crate::KnownLayout#what-is-a-valid-size
    /// [self-unaligned]: Unaligned
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
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: u16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut source = [85, 85];
    /// let _ = ZSTy::try_mut_from_suffix(&mut source[..]); // ⚠ Compile Error!
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &mut [0, 0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let (prefix, packet) = Packet::try_mut_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(prefix, &[0u8][..]);
    ///
    /// prefix[0] = 111;
    /// packet.temperature = 222;
    ///
    /// assert_eq!(bytes, [111, 0xC0, 0xC0, 240, 222, 2, 3, 4, 5, 6, 7]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 77, 240, 0xC0, 0x10][..];
    /// assert!(Packet::try_mut_from_suffix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_suffix(
        source: &mut [u8],
    ) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout + IntoBytes,
    {
        static_assert_dst_is_not_zst!(Self);
        try_mut_from_prefix_suffix(source, CastType::Suffix, None).map(swap)
    }
    /// Attempts to interpret the given `source` as a `&Self` with a DST length
    /// equal to `count`.
    ///
    /// This method attempts to return a reference to `source` interpreted as a
    /// `Self` with `count` trailing elements. If the length of `source` is not
    /// equal to the size of `Self` with `count` elements, if `source` is not
    /// appropriately aligned, or if `source` does not contain a valid instance
    /// of `Self`, this returns `Err`. If [`Self: Unaligned`][self-unaligned],
    /// you can [infallibly discard the alignment error][ConvertError::from].
    ///
    /// [self-unaligned]: Unaligned
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &[0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let packet = Packet::try_ref_from_bytes_with_elems(bytes, 3).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 77, 240, 0xC0, 0xC0][..];
    /// assert!(Packet::try_ref_from_bytes_with_elems(bytes, 3).is_err());
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`try_ref_from_bytes`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: NonZeroU16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = 0xCAFEu16.as_bytes();
    /// let zsty = ZSTy::try_ref_from_bytes_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`try_ref_from_bytes`]: TryFromBytes::try_ref_from_bytes
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_bytes_with_elems(
        source: &[u8],
        count: usize,
    ) -> Result<&Self, TryCastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        match Ptr::from_ref(source).try_cast_into_no_leftover::<Self, BecauseImmutable>(Some(count))
        {
            Ok(source) => match source.try_into_valid() {
                Ok(source) => Ok(source.as_ref()),
                Err(e) => Err(e
                    .map_src(|src| src.as_bytes::<BecauseImmutable>().as_ref())
                    .into()),
            },
            Err(e) => Err(e.map_src(Ptr::as_ref).into()),
        }
    }
    /// Attempts to interpret the prefix of the given `source` as a `&Self` with
    /// a DST length equal to `count`.
    ///
    /// This method attempts to return a reference to the prefix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the remaining bytes. If the length of `source` is less than the size
    /// of `Self` with `count` elements, if `source` is not appropriately
    /// aligned, or if the prefix of `source` does not contain a valid instance
    /// of `Self`, this returns `Err`. If [`Self: Unaligned`][self-unaligned],
    /// you can [infallibly discard the alignment error][ConvertError::from].
    ///
    /// [self-unaligned]: Unaligned
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &[0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7, 8][..];
    ///
    /// let (packet, suffix) = Packet::try_ref_from_prefix_with_elems(bytes, 3).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(suffix, &[8u8][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 77, 240, 0xC0, 0xC0][..];
    /// assert!(Packet::try_ref_from_prefix_with_elems(bytes, 3).is_err());
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`try_ref_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: NonZeroU16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = 0xCAFEu16.as_bytes();
    /// let (zsty, _) = ZSTy::try_ref_from_prefix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`try_ref_from_prefix`]: TryFromBytes::try_ref_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_prefix_with_elems(
        source: &[u8],
        count: usize,
    ) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        try_ref_from_prefix_suffix(source, CastType::Prefix, Some(count))
    }
    /// Attempts to interpret the suffix of the given `source` as a `&Self` with
    /// a DST length equal to `count`.
    ///
    /// This method attempts to return a reference to the suffix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the preceding bytes. If the length of `source` is less than the size
    /// of `Self` with `count` elements, if the suffix of `source` is not
    /// appropriately aligned, or if the suffix of `source` does not contain a
    /// valid instance of `Self`, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][ConvertError::from].
    ///
    /// [self-unaligned]: Unaligned
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, KnownLayout, Immutable)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &[123, 0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let (prefix, packet) = Packet::try_ref_from_suffix_with_elems(bytes, 3).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(prefix, &[123u8][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 77, 240, 0xC0, 0xC0][..];
    /// assert!(Packet::try_ref_from_suffix_with_elems(bytes, 3).is_err());
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`try_ref_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, Immutable, KnownLayout)]
    /// #[repr(C)]
    /// struct ZSTy {
    ///     leading_sized: NonZeroU16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let src = 0xCAFEu16.as_bytes();
    /// let (_, zsty) = ZSTy::try_ref_from_suffix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`try_ref_from_prefix`]: TryFromBytes::try_ref_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn try_ref_from_suffix_with_elems(
        source: &[u8],
        count: usize,
    ) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + Immutable,
    {
        try_ref_from_prefix_suffix(source, CastType::Suffix, Some(count)).map(swap)
    }
    /// Attempts to interpret the given `source` as a `&mut Self` with a DST
    /// length equal to `count`.
    ///
    /// This method attempts to return a reference to `source` interpreted as a
    /// `Self` with `count` trailing elements. If the length of `source` is not
    /// equal to the size of `Self` with `count` elements, if `source` is not
    /// appropriately aligned, or if `source` does not contain a valid instance
    /// of `Self`, this returns `Err`. If [`Self: Unaligned`][self-unaligned],
    /// you can [infallibly discard the alignment error][ConvertError::from].
    ///
    /// [self-unaligned]: Unaligned
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &mut [0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let packet = Packet::try_mut_from_bytes_with_elems(bytes, 3).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    ///
    /// packet.temperature = 111;
    ///
    /// assert_eq!(bytes, [0xC0, 0xC0, 240, 111, 2, 3, 4, 5, 6, 7]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 77, 240, 0xC0, 0xC0][..];
    /// assert!(Packet::try_mut_from_bytes_with_elems(bytes, 3).is_err());
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`try_mut_from_bytes`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: NonZeroU16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut src = 0xCAFEu16;
    /// let src = src.as_mut_bytes();
    /// let zsty = ZSTy::try_mut_from_bytes_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`try_mut_from_bytes`]: TryFromBytes::try_mut_from_bytes
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_bytes_with_elems(
        source: &mut [u8],
        count: usize,
    ) -> Result<&mut Self, TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + IntoBytes,
    {
        match Ptr::from_mut(source).try_cast_into_no_leftover::<Self, BecauseExclusive>(Some(count))
        {
            Ok(source) => match source.try_into_valid() {
                Ok(source) => Ok(source.as_mut()),
                Err(e) => Err(e
                    .map_src(|src| src.as_bytes::<BecauseExclusive>().as_mut())
                    .into()),
            },
            Err(e) => Err(e.map_src(Ptr::as_mut).into()),
        }
    }
    /// Attempts to interpret the prefix of the given `source` as a `&mut Self`
    /// with a DST length equal to `count`.
    ///
    /// This method attempts to return a reference to the prefix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the remaining bytes. If the length of `source` is less than the size
    /// of `Self` with `count` elements, if `source` is not appropriately
    /// aligned, or if the prefix of `source` does not contain a valid instance
    /// of `Self`, this returns `Err`. If [`Self: Unaligned`][self-unaligned],
    /// you can [infallibly discard the alignment error][ConvertError::from].
    ///
    /// [self-unaligned]: Unaligned
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &mut [0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7, 8][..];
    ///
    /// let (packet, suffix) = Packet::try_mut_from_prefix_with_elems(bytes, 3).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(suffix, &[8u8][..]);
    ///
    /// packet.temperature = 111;
    /// suffix[0] = 222;
    ///
    /// assert_eq!(bytes, [0xC0, 0xC0, 240, 111, 2, 3, 4, 5, 6, 7, 222]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 77, 240, 0xC0, 0xC0][..];
    /// assert!(Packet::try_mut_from_prefix_with_elems(bytes, 3).is_err());
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`try_mut_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: NonZeroU16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut src = 0xCAFEu16;
    /// let src = src.as_mut_bytes();
    /// let (zsty, _) = ZSTy::try_mut_from_prefix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`try_mut_from_prefix`]: TryFromBytes::try_mut_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_prefix_with_elems(
        source: &mut [u8],
        count: usize,
    ) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + IntoBytes,
    {
        try_mut_from_prefix_suffix(source, CastType::Prefix, Some(count))
    }
    /// Attempts to interpret the suffix of the given `source` as a `&mut Self`
    /// with a DST length equal to `count`.
    ///
    /// This method attempts to return a reference to the suffix of `source`
    /// interpreted as a `Self` with `count` trailing elements, and a reference
    /// to the preceding bytes. If the length of `source` is less than the size
    /// of `Self` with `count` elements, if the suffix of `source` is not
    /// appropriately aligned, or if the suffix of `source` does not contain a
    /// valid instance of `Self`, this returns `Err`. If [`Self:
    /// Unaligned`][self-unaligned], you can [infallibly discard the alignment
    /// error][ConvertError::from].
    ///
    /// [self-unaligned]: Unaligned
    /// [slice-dst]: KnownLayout#dynamically-sized-types
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    ///     marshmallows: [[u8; 2]],
    /// }
    ///
    /// let bytes = &mut [123, 0xC0, 0xC0, 240, 77, 2, 3, 4, 5, 6, 7][..];
    ///
    /// let (prefix, packet) = Packet::try_mut_from_suffix_with_elems(bytes, 3).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(packet.marshmallows, [[2, 3], [4, 5], [6, 7]]);
    /// assert_eq!(prefix, &[123u8][..]);
    ///
    /// prefix[0] = 111;
    /// packet.temperature = 222;
    ///
    /// assert_eq!(bytes, [111, 0xC0, 0xC0, 240, 222, 2, 3, 4, 5, 6, 7]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 77, 240, 0xC0, 0xC0][..];
    /// assert!(Packet::try_mut_from_suffix_with_elems(bytes, 3).is_err());
    /// ```
    ///
    /// Since an explicit `count` is provided, this method supports types with
    /// zero-sized trailing slice elements. Methods such as [`try_mut_from_prefix`]
    /// which do not take an explicit count do not support such types.
    ///
    /// ```
    /// use core::num::NonZeroU16;
    /// use zerocopy::*;
    /// # use zerocopy_derive::*;
    ///
    /// #[derive(TryFromBytes, IntoBytes, KnownLayout)]
    /// #[repr(C, packed)]
    /// struct ZSTy {
    ///     leading_sized: NonZeroU16,
    ///     trailing_dst: [()],
    /// }
    ///
    /// let mut src = 0xCAFEu16;
    /// let src = src.as_mut_bytes();
    /// let (_, zsty) = ZSTy::try_mut_from_suffix_with_elems(src, 42).unwrap();
    /// assert_eq!(zsty.trailing_dst.len(), 42);
    /// ```
    ///
    /// [`try_mut_from_prefix`]: TryFromBytes::try_mut_from_prefix
    #[must_use = "has no side effects"]
    #[inline]
    fn try_mut_from_suffix_with_elems(
        source: &mut [u8],
        count: usize,
    ) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>
    where
        Self: KnownLayout<PointerMetadata = usize> + IntoBytes,
    {
        try_mut_from_prefix_suffix(source, CastType::Suffix, Some(count)).map(swap)
    }
    /// Attempts to read the given `source` as a `Self`.
    ///
    /// If `source.len() != size_of::<Self>()` or the bytes are not a valid
    /// instance of `Self`, this returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    /// }
    ///
    /// let bytes = &[0xC0, 0xC0, 240, 77][..];
    ///
    /// let packet = Packet::try_read_from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &mut [0x10, 0xC0, 240, 77][..];
    /// assert!(Packet::try_read_from_bytes(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_read_from_bytes(source: &[u8]) -> Result<Self, TryReadError<&[u8], Self>>
    where
        Self: Sized,
    {
        let candidate = match CoreMaybeUninit::<Self>::read_from_bytes(source) {
            Ok(candidate) => candidate,
            Err(e) => {
                return Err(TryReadError::Size(e.with_dst()));
            }
        };
        unsafe { try_read_from(source, candidate) }
    }
    /// Attempts to read a `Self` from the prefix of the given `source`.
    ///
    /// This attempts to read a `Self` from the first `size_of::<Self>()` bytes
    /// of `source`, returning that `Self` and any remaining bytes. If
    /// `source.len() < size_of::<Self>()` or the bytes are not a valid instance
    /// of `Self`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0xC0, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    ///
    /// let (packet, suffix) = Packet::try_read_from_prefix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(suffix, &[0u8, 1, 2, 3, 4, 5, 6][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0x10, 0xC0, 240, 77, 0, 1, 2, 3, 4, 5, 6][..];
    /// assert!(Packet::try_read_from_prefix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_read_from_prefix(source: &[u8]) -> Result<(Self, &[u8]), TryReadError<&[u8], Self>>
    where
        Self: Sized,
    {
        let (candidate, suffix) = match CoreMaybeUninit::<Self>::read_from_prefix(source) {
            Ok(candidate) => candidate,
            Err(e) => {
                return Err(TryReadError::Size(e.with_dst()));
            }
        };
        unsafe { try_read_from(source, candidate).map(|slf| (slf, suffix)) }
    }
    /// Attempts to read a `Self` from the suffix of the given `source`.
    ///
    /// This attempts to read a `Self` from the last `size_of::<Self>()` bytes
    /// of `source`, returning that `Self` and any preceding bytes. If
    /// `source.len() < size_of::<Self>()` or the bytes are not a valid instance
    /// of `Self`, it returns `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![allow(non_camel_case_types)] // For C0::xC0
    /// use zerocopy::TryFromBytes;
    /// # use zerocopy_derive::*;
    ///
    /// // The only valid value of this type is the byte `0xC0`
    /// #[derive(TryFromBytes)]
    /// #[repr(u8)]
    /// enum C0 { xC0 = 0xC0 }
    ///
    /// // The only valid value of this type is the bytes `0xC0C0`.
    /// #[derive(TryFromBytes)]
    /// #[repr(C)]
    /// struct C0C0(C0, C0);
    ///
    /// #[derive(TryFromBytes)]
    /// #[repr(C)]
    /// struct Packet {
    ///     magic_number: C0C0,
    ///     mug_size: u8,
    ///     temperature: u8,
    /// }
    ///
    /// // These are more bytes than are needed to encode a `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 0xC0, 0xC0, 240, 77][..];
    ///
    /// let (prefix, packet) = Packet::try_read_from_suffix(bytes).unwrap();
    ///
    /// assert_eq!(packet.mug_size, 240);
    /// assert_eq!(packet.temperature, 77);
    /// assert_eq!(prefix, &[0u8, 1, 2, 3, 4, 5][..]);
    ///
    /// // These bytes are not valid instance of `Packet`.
    /// let bytes = &[0, 1, 2, 3, 4, 5, 0x10, 0xC0, 240, 77][..];
    /// assert!(Packet::try_read_from_suffix(bytes).is_err());
    /// ```
    #[must_use = "has no side effects"]
    #[inline]
    fn try_read_from_suffix(source: &[u8]) -> Result<(&[u8], Self), TryReadError<&[u8], Self>>
    where
        Self: Sized,
    {
        let (prefix, candidate) = match CoreMaybeUninit::<Self>::read_from_suffix(source) {
            Ok(candidate) => candidate,
            Err(e) => {
                return Err(TryReadError::Size(e.with_dst()));
            }
        };
        unsafe { try_read_from(source, candidate).map(|slf| (prefix, slf)) }
    }
}
