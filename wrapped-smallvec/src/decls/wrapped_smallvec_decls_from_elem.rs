use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This function is used in the [`smallvec`] macro.
/// It is recommended to use the macro instead of using thís function.
#[doc(hidden)]
#[track_caller]
pub fn from_elem<T: Clone, const N: usize>(elem: T, n: usize) -> SmallVec<T, N> {
    if n > SmallVec::<T, N>::inline_size() {
        SmallVec::<T, N>::from_vec(vec![elem; n])
    } else {
        #[cfg(feature = "specialization")]
        {
            unsafe { <SmallVec<T, N> as spec_traits::SpecFromElem<T>>::spec_from_elem(elem, n) }
        }
        #[cfg(not(feature = "specialization"))]
        {
            unsafe { SmallVec::<T, N>::from_elem_fallback(elem, n) }
        }
    }
}
