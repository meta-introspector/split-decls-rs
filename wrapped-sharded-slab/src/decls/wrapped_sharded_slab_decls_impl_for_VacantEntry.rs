use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, C: cfg::Config> VacantEntry<'_, T, C> {
    /// Insert a value in the entry.
    ///
    /// To get the integer index at which this value will be inserted, use
    /// [`key`] prior to calling `insert`.
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
    ///
    /// [`key`]: VacantEntry::key
    pub fn insert(mut self, val: T) {
        let value = unsafe { self.inner.value_mut() };
        debug_assert!(
            value.is_none(), "tried to insert to a slot that already had a value!"
        );
        *value = Some(val);
        let _released = unsafe { self.inner.release() };
        debug_assert!(
            ! _released, "removing a value before it was inserted should be a no-op"
        )
    }
    /// Return the integer index at which this entry will be inserted.
    ///
    /// A value stored in this entry will be associated with this key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sharded_slab::*;
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
    pub fn key(&self) -> usize {
        self.key
    }
}
