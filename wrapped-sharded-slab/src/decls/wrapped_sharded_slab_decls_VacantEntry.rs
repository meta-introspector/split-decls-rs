use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A handle to a vacant entry in a [`Slab`].
///
/// `VacantEntry` allows constructing values with the key that they will be
/// assigned to.
///
/// # Examples
///
/// ```
/// # use sharded_slab::Slab;
/// let mut slab = Slab::new();
///
/// let hello = {
///     let entry = slab.vacant_entry().unwrap();
///     let key = entry.key();
///
///     entry.insert((key, "hello"));
///     key
/// };
///
/// assert_eq!(hello, slab.get(hello).unwrap().0);
/// assert_eq!("hello", slab.get(hello).unwrap().1);
/// ```
#[derive(Debug)]
pub struct VacantEntry<'a, T, C: cfg::Config = DefaultConfig> {
    inner: page::slot::InitGuard<Option<T>, C>,
    key: usize,
    _lt: PhantomData<&'a ()>,
}
