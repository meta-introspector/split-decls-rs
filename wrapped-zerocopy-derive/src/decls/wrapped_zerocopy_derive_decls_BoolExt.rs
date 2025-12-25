use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused)]
trait BoolExt {
    fn then_some<T>(self, t: T) -> Option<T>;
}
