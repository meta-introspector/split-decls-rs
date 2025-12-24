use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Helper method which does the same thing as rustc 1.20's
/// `Option::get_or_insert_with`. This method is used to keep backwards
/// compatibility with rustc 1.15.
fn get_or_insert_with<T, F>(opt: &mut Option<T>, f: F) -> &mut T
where
    F: FnOnce() -> T,
{
    if opt.is_none() {
        *opt = Some(f());
    }
    match opt {
        Some(v) => v,
        None => unreachable!(),
    }
}
