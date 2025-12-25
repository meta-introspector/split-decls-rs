use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PointerMetadata for usize {
    #[inline]
    fn from_elem_count(elems: usize) -> usize {
        elems
    }
    #[inline]
    fn size_for_metadata(self, layout: DstLayout) -> Option<usize> {
        match layout.size_info {
            SizeInfo::SliceDst(TrailingSliceLayout { offset, elem_size }) => {
                let slice_len = elem_size.checked_mul(self)?;
                let without_padding = offset.checked_add(slice_len)?;
                without_padding
                    .checked_add(util::padding_needed_for(without_padding, layout.align))
            }
            SizeInfo::Sized { .. } => None,
        }
    }
}
