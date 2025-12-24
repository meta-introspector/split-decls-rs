use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod private {
    pub trait Sealed {}
    #[cfg(not(feature = "cjk"))]
    impl Sealed for char {}
    #[cfg(not(feature = "cjk"))]
    impl Sealed for str {}
    #[cfg(feature = "cjk")]
    impl<T: ?Sized> Sealed for T {}
}
