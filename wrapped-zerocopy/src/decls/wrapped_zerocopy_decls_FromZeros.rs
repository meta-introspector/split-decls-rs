use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Types for which a sequence of `0` bytes is a valid instance.
///
/// Any memory region of the appropriate length which is guaranteed to contain
/// only zero bytes can be viewed as any `FromZeros` type with no runtime
/// overhead. This is useful whenever memory is known to be in a zeroed state,
/// such memory returned from some allocation routines.
///
/// # Warning: Padding bytes
///
/// Note that, when a value is moved or copied, only the non-padding bytes of
/// that value are guaranteed to be preserved. It is unsound to assume that
/// values written to padding bytes are preserved after a move or copy. For more
/// details, see the [`FromBytes` docs][frombytes-warning-padding-bytes].
///
/// [frombytes-warning-padding-bytes]: FromBytes#warning-padding-bytes
///
/// # Implementation
///
/// **Do not implement this trait yourself!** Instead, use
/// [`#[derive(FromZeros)]`][derive]; e.g.:
///
/// ```
/// # use zerocopy_derive::{FromZeros, Immutable};
/// #[derive(FromZeros)]
/// struct MyStruct {
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromZeros)]
/// #[repr(u8)]
/// enum MyEnum {
/// #   Variant0,
/// # /*
///     ...
/// # */
/// }
///
/// #[derive(FromZeros, Immutable)]
/// union MyUnion {
/// #   variant: u8,
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// This derive performs a sophisticated, compile-time safety analysis to
/// determine whether a type is `FromZeros`.
///
/// # Safety
///
/// *This section describes what is required in order for `T: FromZeros`, and
/// what unsafe code may assume of such types. If you don't plan on implementing
/// `FromZeros` manually, and you don't plan on writing unsafe code that
/// operates on `FromZeros` types, then you don't need to read this section.*
///
/// If `T: FromZeros`, then unsafe code may assume that it is sound to produce a
/// `T` whose bytes are all initialized to zero. If a type is marked as
/// `FromZeros` which violates this contract, it may cause undefined behavior.
///
/// `#[derive(FromZeros)]` only permits [types which satisfy these
/// requirements][derive-analysis].
///
#[cfg_attr(
    feature = "derive",
    doc = "[derive]: zerocopy_derive::FromZeros",
    doc = "[derive-analysis]: zerocopy_derive::FromZeros#analysis"
)]
#[cfg_attr(
    not(feature = "derive"),
    doc = concat!(
        "[derive]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.FromZeros.html"
    ),
    doc = concat!(
        "[derive-analysis]: https://docs.rs/zerocopy/",
        env!("CARGO_PKG_VERSION"),
        "/zerocopy/derive.FromZeros.html#analysis"
    ),
)]
#[cfg_attr(
    zerocopy_diagnostic_on_unimplemented_1_78_0,
    diagnostic::on_unimplemented(
        note = "Consider adding `#[derive(FromZeros)]` to `{Self}`"
    )
)]
pub unsafe trait FromZeros: TryFromBytes {
    #[doc(hidden)]
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized;
    /// Overwrites `self` with zeros.
    ///
    /// Sets every byte in `self` to 0. While this is similar to doing `*self =
    /// Self::new_zeroed()`, it differs in that `zero` does not semantically
    /// drop the current value and replace it with a new one — it simply
    /// modifies the bytes of the existing value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zerocopy::FromZeros;
    /// # use zerocopy_derive::*;
    /// #
    /// #[derive(FromZeros)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let mut header = PacketHeader {
    ///     src_port: 100u16.to_be_bytes(),
    ///     dst_port: 200u16.to_be_bytes(),
    ///     length: 300u16.to_be_bytes(),
    ///     checksum: 400u16.to_be_bytes(),
    /// };
    ///
    /// header.zero();
    ///
    /// assert_eq!(header.src_port, [0, 0]);
    /// assert_eq!(header.dst_port, [0, 0]);
    /// assert_eq!(header.length, [0, 0]);
    /// assert_eq!(header.checksum, [0, 0]);
    /// ```
    #[inline(always)]
    fn zero(&mut self) {
        let slf: *mut Self = self;
        let len = mem::size_of_val(self);
        unsafe { ptr::write_bytes(slf.cast::<u8>(), 0, len) };
    }
    /// Creates an instance of `Self` from zeroed bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use zerocopy::FromZeros;
    /// # use zerocopy_derive::*;
    /// #
    /// #[derive(FromZeros)]
    /// #[repr(C)]
    /// struct PacketHeader {
    ///     src_port: [u8; 2],
    ///     dst_port: [u8; 2],
    ///     length: [u8; 2],
    ///     checksum: [u8; 2],
    /// }
    ///
    /// let header: PacketHeader = FromZeros::new_zeroed();
    ///
    /// assert_eq!(header.src_port, [0, 0]);
    /// assert_eq!(header.dst_port, [0, 0]);
    /// assert_eq!(header.length, [0, 0]);
    /// assert_eq!(header.checksum, [0, 0]);
    /// ```
    #[must_use = "has no side effects"]
    #[inline(always)]
    fn new_zeroed() -> Self
    where
        Self: Sized,
    {
        unsafe { mem::zeroed() }
    }
    /// Creates a `Box<Self>` from zeroed bytes.
    ///
    /// This function is useful for allocating large values on the heap and
    /// zero-initializing them, without ever creating a temporary instance of
    /// `Self` on the stack. For example, `<[u8; 1048576]>::new_box_zeroed()`
    /// will allocate `[u8; 1048576]` directly on the heap; it does not require
    /// storing `[u8; 1048576]` in a temporary variable on the stack.
    ///
    /// On systems that use a heap implementation that supports allocating from
    /// pre-zeroed memory, using `new_box_zeroed` (or related functions) may
    /// have performance benefits.
    ///
    /// # Errors
    ///
    /// Returns an error on allocation failure. Allocation failure is guaranteed
    /// never to cause a panic or an abort.
    #[must_use = "has no side effects (other than allocation)"]
    #[cfg(any(feature = "alloc", test))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[inline]
    fn new_box_zeroed() -> Result<Box<Self>, AllocError>
    where
        Self: Sized,
    {
        let layout = Layout::new::<Self>();
        if layout.size() == 0 {
            return Ok(unsafe { Box::from_raw(NonNull::dangling().as_ptr()) });
        }
        #[allow(clippy::undocumented_unsafe_blocks)]
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout).cast::<Self>() };
        if ptr.is_null() {
            return Err(AllocError);
        }
        #[allow(clippy::undocumented_unsafe_blocks)] Ok(unsafe { Box::from_raw(ptr) })
    }
    /// Creates a `Box<[Self]>` (a boxed slice) from zeroed bytes.
    ///
    /// This function is useful for allocating large values of `[Self]` on the
    /// heap and zero-initializing them, without ever creating a temporary
    /// instance of `[Self; _]` on the stack. For example,
    /// `u8::new_box_slice_zeroed(1048576)` will allocate the slice directly on
    /// the heap; it does not require storing the slice on the stack.
    ///
    /// On systems that use a heap implementation that supports allocating from
    /// pre-zeroed memory, using `new_box_slice_zeroed` may have performance
    /// benefits.
    ///
    /// If `Self` is a zero-sized type, then this function will return a
    /// `Box<[Self]>` that has the correct `len`. Such a box cannot contain any
    /// actual information, but its `len()` property will report the correct
    /// value.
    ///
    /// # Errors
    ///
    /// Returns an error on allocation failure. Allocation failure is
    /// guaranteed never to cause a panic or an abort.
    #[must_use = "has no side effects (other than allocation)"]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[inline]
    fn new_box_zeroed_with_elems(count: usize) -> Result<Box<Self>, AllocError>
    where
        Self: KnownLayout<PointerMetadata = usize>,
    {
        unsafe { crate::util::new_box(count, alloc::alloc::alloc_zeroed) }
    }
    #[deprecated(
        since = "0.8.0",
        note = "renamed to `FromZeros::new_box_zeroed_with_elems`"
    )]
    #[doc(hidden)]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[must_use = "has no side effects (other than allocation)"]
    #[inline(always)]
    fn new_box_slice_zeroed(len: usize) -> Result<Box<[Self]>, AllocError>
    where
        Self: Sized,
    {
        <[Self]>::new_box_zeroed_with_elems(len)
    }
    /// Creates a `Vec<Self>` from zeroed bytes.
    ///
    /// This function is useful for allocating large values of `Vec`s and
    /// zero-initializing them, without ever creating a temporary instance of
    /// `[Self; _]` (or many temporary instances of `Self`) on the stack. For
    /// example, `u8::new_vec_zeroed(1048576)` will allocate directly on the
    /// heap; it does not require storing intermediate values on the stack.
    ///
    /// On systems that use a heap implementation that supports allocating from
    /// pre-zeroed memory, using `new_vec_zeroed` may have performance benefits.
    ///
    /// If `Self` is a zero-sized type, then this function will return a
    /// `Vec<Self>` that has the correct `len`. Such a `Vec` cannot contain any
    /// actual information, but its `len()` property will report the correct
    /// value.
    ///
    /// # Errors
    ///
    /// Returns an error on allocation failure. Allocation failure is
    /// guaranteed never to cause a panic or an abort.
    #[must_use = "has no side effects (other than allocation)"]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "alloc")))]
    #[inline(always)]
    fn new_vec_zeroed(len: usize) -> Result<Vec<Self>, AllocError>
    where
        Self: Sized,
    {
        <[Self]>::new_box_zeroed_with_elems(len).map(Into::into)
    }
    /// Extends a `Vec<Self>` by pushing `additional` new items onto the end of
    /// the vector. The new items are initialized with zeros.
    #[cfg(zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(all(rust = "1.57.0", feature = "alloc"))))]
    #[inline(always)]
    fn extend_vec_zeroed(v: &mut Vec<Self>, additional: usize) -> Result<(), AllocError>
    where
        Self: Sized,
    {
        <Self as FromZeros>::insert_vec_zeroed(v, v.len(), additional)
    }
    /// Inserts `additional` new items into `Vec<Self>` at `position`. The new
    /// items are initialized with zeros.
    ///
    /// # Panics
    ///
    /// Panics if `position > v.len()`.
    #[cfg(zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)]
    #[cfg(feature = "alloc")]
    #[cfg_attr(doc_cfg, doc(cfg(all(rust = "1.57.0", feature = "alloc"))))]
    #[inline]
    fn insert_vec_zeroed(
        v: &mut Vec<Self>,
        position: usize,
        additional: usize,
    ) -> Result<(), AllocError>
    where
        Self: Sized,
    {
        assert!(position <= v.len());
        v.try_reserve(additional).map_err(|_| AllocError)?;
        unsafe {
            let ptr = v.as_mut_ptr();
            #[allow(clippy::arithmetic_side_effects)]
            ptr.add(position)
                .copy_to(ptr.add(position + additional), v.len() - position);
            ptr.add(position).write_bytes(0, additional);
            #[allow(clippy::arithmetic_side_effects)] v.set_len(v.len() + additional);
        }
        Ok(())
    }
}
