use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! into_impl {
    ($to:ty) => {
        impl<'a> Into<$to> for UniCase<$to> {
            fn into(self) -> $to {
                self.into_inner()
            }
        }
    };
}
