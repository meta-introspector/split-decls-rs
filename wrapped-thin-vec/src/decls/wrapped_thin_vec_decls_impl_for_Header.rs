use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Header {
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    fn len(&self) -> usize {
        self._len as usize
    }
    #[inline]
    fn set_len(&mut self, len: usize) {
        self._len = assert_size(len);
    }
    fn cap(&self) -> usize {
        unpack_capacity(self._cap)
    }
    fn set_cap_and_auto(&mut self, cap: usize, is_auto: bool) {
        debug_assert_eq!(
            unpack_capacity(pack_capacity_and_auto(cap as SizeType, is_auto)), cap
        );
        self._cap = pack_capacity_and_auto(assert_size(cap), is_auto);
    }
    #[inline]
    fn is_auto(&self) -> bool {
        is_auto(self._cap)
    }
}
