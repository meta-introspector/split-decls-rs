use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Interprets the given affix of the given bytes as a `&mut Self` without
/// copying.
///
/// This method computes the largest possible size of `Self` that can fit in the
/// prefix or suffix bytes of `source`, then attempts to return both a reference
/// to those bytes interpreted as a `Self`, and a reference to the excess bytes.
/// If there are insufficient bytes, or if that affix of `source` is not
/// appropriately aligned, this returns `Err`.
#[inline(always)]
fn mut_from_prefix_suffix<T: FromBytes + IntoBytes + KnownLayout + ?Sized>(
    source: &mut [u8],
    meta: Option<T::PointerMetadata>,
    cast_type: CastType,
) -> Result<(&mut T, &mut [u8]), CastError<&mut [u8], T>> {
    let (slf, prefix_suffix) = Ptr::from_mut(source)
        .try_cast_into::<_, BecauseExclusive>(cast_type, meta)
        .map_err(|err| err.map_src(|s| s.as_mut()))?;
    Ok((slf.recall_validity::<_, (_, (_, _))>().as_mut(), prefix_suffix.as_mut()))
}
