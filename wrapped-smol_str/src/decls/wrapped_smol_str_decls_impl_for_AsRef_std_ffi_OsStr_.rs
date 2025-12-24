use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl AsRef<std::ffi::OsStr> for SmolStr {
    #[inline(always)]
    fn as_ref(&self) -> &std::ffi::OsStr {
        AsRef::<std::ffi::OsStr>::as_ref(self.as_str())
    }
}
