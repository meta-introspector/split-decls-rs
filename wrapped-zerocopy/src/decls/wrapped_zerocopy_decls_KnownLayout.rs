use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Indicates that zerocopy can reason about certain aspects of a type's layout.
///
/// This trait is required by many of zerocopy's APIs. It supports sized types,
/// slices, and [slice DSTs](#dynamically-sized-types).
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(KnownLayout)]`][derive]; e.g.:
///
/// ```
/// # use zerocopy_derive::KnownLayout;
/// #[derive(KnownLayout)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(KnownLayout)]
/// enum MyEnum {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(KnownLayout)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated analysis to deduce the layout
/// characteristics of types. You **must** implement this trait via the derive.
///
/// # Dynamically-sized types
///
/// `KnownLayout` supports slice-based dynamically sized types ("slice DSTs").
///
/// A slice DST is a type whose trailing field is either a slice or another
/// slice DST, rather than a type with fixed size. For example:
///
/// ```
/// #[repr(C)]
/// struct PacketHeader {
/// # /*
///     ...
/// # */
/// }
///
/// #[repr(C)]
/// struct Packet {
///     header: PacketHeader,
///     body: [u8],
/// }
/// ```
///
/// It can be useful to think of slice DSTs as a generalization of slices - in
/// other words, a normal slice is just the special case of a slice DST with
/// zero leading fields. In particular:
/// - Like slices, slice DSTs can have different lengths at runtime
/// - Like slices, slice DSTs cannot be passed by-value, but only by reference
///   or via other indirection such as `Box`
/// - Like slices, a reference (or `Box`, or other pointer type) to a slice DST
///   encodes the number of elements in the trailing slice field
///
/// ## Slice DST layout
///
/// Just like other composite Rust types, the layout of a slice DST is not
/// well-defined unless it is specified using an explicit `#[repr(...)]`
/// attribute such as `#[repr(C)]`. [Other representations are
/// supported][reprs], but in this section, we'll use `#[repr(C)]` as our
/// example.
///
/// A `#[repr(C)]` slice DST is laid out [just like sized `#[repr(C)]`
/// types][repr-c-structs], but the presence of a variable-length field
/// introduces the possibility of *dynamic padding*. In particular, it may be
/// necessary to add trailing padding *after* the trailing slice field in order
/// to satisfy the outer type's alignment, and the amount of padding required
/// may be a function of the length of the trailing slice field. This is just a
/// natural consequence of the normal `#[repr(C)]` rules applied to slice DSTs,
/// but it can result in surprising behavior. For example, consider the
/// following type:
///
/// ```
/// #[repr(C)]
/// struct Foo {
///     a: u32,
///     b: u8,
///     z: [u16],
/// }
/// ```
///
/// Assuming that `u32` has alignment 4 (this is not true on all platforms),
/// then `Foo` has alignment 4 as well. Here is the smallest possible value for
/// `Foo`:
///
/// ```text
/// byte offset | 01234567
///       field | aaaab---
///                    ><
/// ```
///
/// In this value, `z` has length 0. Abiding by `#[repr(C)]`, the lowest offset
/// that we can place `z` at is 5, but since `z` has alignment 2, we need to
/// round up to offset 6. This means that there is one byte of padding between
/// `b` and `z`, then 0 bytes of `z` itself (denoted `><` in this diagram), and
/// then two bytes of padding after `z` in order to satisfy the overall
/// alignment of `Foo`. The size of this instance is 8 bytes.
///
/// What about if `z` has length 1?
///
/// ```text
/// byte offset | 01234567
///       field | aaaab-zz
/// ```
///
/// In this instance, `z` has length 1, and thus takes up 2 bytes. That means
/// that we no longer need padding after `z` in order to satisfy `Foo`'s
/// alignment. We've now seen two different values of `Foo` with two different
/// lengths of `z`, but they both have the same size - 8 bytes.
///
/// What about if `z` has length 2?
///
/// ```text
/// byte offset | 012345678901
///       field | aaaab-zzzz--
/// ```
///
/// Now `z` has length 2, and thus takes up 4 bytes. This brings our un-padded
/// size to 10, and so we now need another 2 bytes of padding after `z` to
/// satisfy `Foo`'s alignment.
///
/// Again, all of this is just a logical consequence of the `#[repr(C)]` rules
/// applied to slice DSTs, but it can be surprising that the amount of trailing
/// padding becomes a function of the trailing slice field's length, and thus
/// can only be computed at runtime.
///
/// [reprs]: https://doc.rust-lang.org/reference/type-layout.html#representations
/// [repr-c-structs]: https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
///
/// ## What is a valid size?
///
/// There are two places in zerocopy's API that we refer to "a valid size" of a
/// type. In normal casts or conversions, where the source is a byte slice, we
/// need to know whether the source byte slice is a valid size of the
/// destination type. In prefix or suffix casts, we need to know whether *there
/// exists* a valid size of the destination type which fits in the source byte
/// slice and, if so, what the largest such size is.
///
/// As outlined above, a slice DST's size is defined by the number of elements
/// in its trailing slice field. However, there is not necessarily a 1-to-1
/// mapping between trailing slice field length and overall size. As we saw in
/// the previous section with the type `Foo`, instances with both 0 and 1
/// elements in the trailing `z` field result in a `Foo` whose size is 8 bytes.
///
/// When we say "x is a valid size of `T`", we mean one of two things:
/// - If `T: Sized`, then we mean that `x == size_of::<T>()`
/// - If `T` is a slice DST, then we mean that there exists a `len` such that the instance of
///   `T` with `len` trailing slice elements has size `x`
///
/// When we say "largest possible size of `T` that fits in a byte slice", we
/// mean one of two things:
/// - If `T: Sized`, then we mean `size_of::<T>()` if the byte slice is at least
///   `size_of::<T>()` bytes long
/// - If `T` is a slice DST, then we mean to consider all values, `len`, such
///   that the instance of `T` with `len` trailing slice elements fits in the
///   byte slice, and to choose the largest such `len`, if any
///
///
/// # Safety
///
/// This trait does not convey any safety guarantees to code outside this crate.
///
/// You must not rely on the `#[doc(hidden)]` internals of `KnownLayout`. Future
/// releases of zerocopy may make backwards-breaking changes to these items,
/// including changes that only affect soundness, which may cause code which
/// uses those items to silently become unsound.
///
#[cfg_attr(feature = "derive", doc = "[derive]: zerocopy_derive::KnownLayout")]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.KnownLayout.html"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(
        note = "Consider adding `#[derive(KnownLayout)]` to `{Self}`"
    )
)]
pub unsafe trait KnownLayout {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
    /// The type of metadata stored in a pointer to `Self`.
    ///
    /// This is `()` for sized types and `usize` for slice DSTs.
    type PointerMetadata: PointerMetadata;
    /// A maybe-uninitialized analog of `Self`
    ///
    /// # Safety
    ///
    /// `Self::LAYOUT` and `Self::MaybeUninit::LAYOUT` are identical.
    /// `Self::MaybeUninit` admits uninitialized bytes in all positions.
    #[doc(hidden)]
    type MaybeUninit: ?Sized + KnownLayout<PointerMetadata = Self::PointerMetadata>;
    /// The layout of `Self`.
    ///
    /// # Safety
    ///
    /// Callers may assume that `LAYOUT` accurately reflects the layout of
    /// `Self`. In particular:
    /// - `LAYOUT.align` is equal to `Self`'s alignment
    /// - If `Self: Sized`, then `LAYOUT.size_info == SizeInfo::Sized { size }`
    ///   where `size == size_of::<Self>()`
    /// - If `Self` is a slice DST, then `LAYOUT.size_info ==
    ///   SizeInfo::SliceDst(slice_layout)` where:
    ///   - The size, `size`, of an instance of `Self` with `elems` trailing
    ///     slice elements is equal to `slice_layout.offset +
    ///     slice_layout.elem_size * elems` rounded up to the nearest multiple
    ///     of `LAYOUT.align`
    ///   - For such an instance, any bytes in the range `[slice_layout.offset +
    ///     slice_layout.elem_size * elems, size)` are padding and must not be
    ///     assumed to be initialized
    #[doc(hidden)]
    const LAYOUT: DstLayout;
    /// SAFETY: The returned pointer has the same address and provenance as
    /// `bytes`. If `Self` is a DST, the returned pointer's referent has `elems`
    /// elements in its trailing slice.
    #[doc(hidden)]
    fn raw_from_ptr_len(
        bytes: NonNull<u8>,
        meta: Self::PointerMetadata,
    ) -> NonNull<Self>;
    /// Extracts the metadata from a pointer to `Self`.
    ///
    /// # Safety
    ///
    /// `pointer_to_metadata` always returns the correct metadata stored in
    /// `ptr`.
    #[doc(hidden)]
    fn pointer_to_metadata(ptr: *mut Self) -> Self::PointerMetadata;
    /// Computes the length of the byte range addressed by `ptr`.
    ///
    /// Returns `None` if the resulting length would not fit in an `usize`.
    ///
    /// # Safety
    ///
    /// Callers may assume that `size_of_val_raw` always returns the correct
    /// size.
    ///
    /// Callers may assume that, if `ptr` addresses a byte range whose length
    /// fits in an `usize`, this will return `Some`.
    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    fn size_of_val_raw(ptr: NonNull<Self>) -> Option<usize> {
        let meta = Self::pointer_to_metadata(ptr.as_ptr());
        Self::size_for_metadata(meta)
    }
    #[doc(hidden)]
    #[must_use]
    #[inline(always)]
    fn raw_dangling() -> NonNull<Self> {
        let meta = Self::PointerMetadata::from_elem_count(0);
        Self::raw_from_ptr_len(NonNull::dangling(), meta)
    }
    /// Computes the size of an object of type `Self` with the given pointer
    /// metadata.
    ///
    /// # Safety
    ///
    /// `size_for_metadata` promises to return `None` if and only if the
    /// resulting size would not fit in a `usize`. Note that the returned size
    /// could exceed the actual maximum valid size of an allocated object,
    /// `isize::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zerocopy::KnownLayout;
    ///
    /// assert_eq!(u8::size_for_metadata(()), Some(1));
    /// assert_eq!(u16::size_for_metadata(()), Some(2));
    /// assert_eq!(<[u8]>::size_for_metadata(42), Some(42));
    /// assert_eq!(<[u16]>::size_for_metadata(42), Some(84));
    ///
    /// // This size exceeds the maximum valid object size (`isize::MAX`):
    /// assert_eq!(<[u8]>::size_for_metadata(usize::MAX), Some(usize::MAX));
    ///
    /// // This size, if computed, would exceed `usize::MAX`:
    /// assert_eq!(<[u16]>::size_for_metadata(usize::MAX), None);
    /// ```
    #[inline(always)]
    fn size_for_metadata(meta: Self::PointerMetadata) -> Option<usize> {
        meta.size_for_metadata(Self::LAYOUT)
    }
}
