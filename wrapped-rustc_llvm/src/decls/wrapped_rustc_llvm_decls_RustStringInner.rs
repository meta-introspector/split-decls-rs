use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Underlying implementation of [`RustString`].
///
/// Having two separate types makes it possible to use the opaque [`RustString`]
/// in FFI signatures without `improper_ctypes` warnings. This is a workaround
/// for the fact that there is no way to opt out of `improper_ctypes` when
/// _declaring_ a type (as opposed to using that type).
#[derive(Default)]
struct RustStringInner {
    bytes: RefCell<Vec<u8>>,
}
