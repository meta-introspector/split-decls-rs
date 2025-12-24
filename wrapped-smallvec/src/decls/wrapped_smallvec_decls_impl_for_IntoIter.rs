use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> IntoIter<T, N> {
    #[inline]
    const fn is_zst() -> bool {
        size_of::<T>() == 0
    }
    #[inline]
    const fn as_ptr(&self) -> *const T {
        let on_heap = self.end.on_heap(Self::is_zst());
        if on_heap {
            unsafe { self.raw.as_ptr_heap() }
        } else {
            self.raw.as_ptr_inline()
        }
    }
    #[inline]
    const fn as_mut_ptr(&mut self) -> *mut T {
        let on_heap = self.end.on_heap(Self::is_zst());
        if on_heap {
            unsafe { self.raw.as_mut_ptr_heap() }
        } else {
            self.raw.as_mut_ptr_inline()
        }
    }
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe {
            let ptr = self.as_ptr();
            core::slice::from_raw_parts(
                ptr.add(self.begin),
                self.end.value(Self::is_zst()) - self.begin,
            )
        }
    }
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let ptr = self.as_mut_ptr();
            core::slice::from_raw_parts_mut(
                ptr.add(self.begin),
                self.end.value(Self::is_zst()) - self.begin,
            )
        }
    }
}
