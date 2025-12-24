use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl UnicodeWidthStr for str {
    #[inline]
    fn width(&self) -> usize {
        tables::str_width(self)
    }
    #[cfg(feature = "cjk")]
    #[inline]
    fn width_cjk(&self) -> usize {
        tables::str_width_cjk(self)
    }
}
