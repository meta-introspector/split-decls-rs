use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, C: cfg::Config> Slab<T, C> {
    /// The number of bits in each index which are used by the slab.
    ///
    /// If other data is packed into the `usize` indices returned by
    /// [`Slab::insert`], user code is free to use any bits higher than the
    /// `USED_BITS`-th bit freely.
    ///
    /// This is determined by the [`Config`] type that configures the slab's
    /// parameters. By default, all bits are used; this can be changed by
    /// overriding the [`Config::RESERVED_BITS`][res] constant.
    ///
    /// [res]: crate::Config#RESERVED_BITS
    pub const USED_BITS: usize = C::USED_BITS;
    /// Inserts a value into the slab, returning the integer index at which that
    /// value was inserted. This index can then be used to access the entry.
    ///
    /// If this function returns `None`, then the shard for the current thread
    /// is full and no items can be added until some are removed, or the maximum
    /// number of shards has been reached.
    ///
    /// # Examples
    /// ```rust
    /// # use sharded_slab::Slab;
    /// let slab = Slab::new();
    ///
    /// let key = slab.insert("hello world").unwrap();
    /// assert_eq!(slab.get(key).unwrap(), "hello world");
    /// ```
    pub fn insert(&self, value: T) -> Option<usize> {
        let (tid, shard) = self.shards.current();
        test_println!("insert {:?}", tid);
        let mut value = Some(value);
        shard
            .init_with(|idx, slot| {
                let gen = slot.insert(&mut value)?;
                Some(gen.pack(idx))
            })
            .map(|idx| tid.pack(idx))
    }
    /// Return a handle to a vacant entry allowing for further manipulation.
    ///
    /// This function is useful when creating values that must contain their
    /// slab index. The returned [`VacantEntry`] reserves a slot in the slab and
    /// is able to return the index of the entry.
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
    pub fn vacant_entry(&self) -> Option<VacantEntry<'_, T, C>> {
        let (tid, shard) = self.shards.current();
        test_println!("vacant_entry {:?}", tid);
        shard
            .init_with(|idx, slot| {
                let inner = slot.init()?;
                let key = inner.generation().pack(tid.pack(idx));
                Some(VacantEntry {
                    inner,
                    key,
                    _lt: PhantomData,
                })
            })
    }
    /// Remove the value at the given index in the slab, returning `true` if a
    /// value was removed.
    ///
    /// Unlike [`take`], this method does _not_ block the current thread until
    /// the value can be removed. Instead, if another thread is currently
    /// accessing that value, this marks it to be removed by that thread when it
    /// finishes accessing the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let slab = sharded_slab::Slab::new();
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// // Remove the item from the slab.
    /// assert!(slab.remove(key));
    ///
    /// // Now, the slot is empty.
    /// assert!(!slab.contains(key));
    /// ```
    ///
    /// ```rust
    /// use std::sync::Arc;
    ///
    /// let slab = Arc::new(sharded_slab::Slab::new());
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// let slab2 = slab.clone();
    /// let thread2 = std::thread::spawn(move || {
    ///     // Depending on when this thread begins executing, the item may
    ///     // or may not have already been removed...
    ///     if let Some(item) = slab2.get(key) {
    ///         assert_eq!(item, "hello world");
    ///     }
    /// });
    ///
    /// // The item will be removed by thread2 when it finishes accessing it.
    /// assert!(slab.remove(key));
    ///
    /// thread2.join().unwrap();
    /// assert!(!slab.contains(key));
    /// ```
    /// [`take`]: Slab::take
    pub fn remove(&self, idx: usize) -> bool {
        let tid = C::unpack_tid(idx);
        test_println!("rm_deferred {:?}", tid);
        let shard = self.shards.get(tid.as_usize());
        if tid.is_current() {
            shard.map(|shard| shard.remove_local(idx)).unwrap_or(false)
        } else {
            shard.map(|shard| shard.remove_remote(idx)).unwrap_or(false)
        }
    }
    /// Removes the value associated with the given key from the slab, returning
    /// it.
    ///
    /// If the slab does not contain a value for that key, `None` is returned
    /// instead.
    ///
    /// If the value associated with the given key is currently being
    /// accessed by another thread, this method will block the current thread
    /// until the item is no longer accessed. If this is not desired, use
    /// [`remove`] instead.
    ///
    /// **Note**: This method blocks the calling thread by spinning until the
    /// currently outstanding references are released. Spinning for long periods
    /// of time can result in high CPU time and power consumption. Therefore,
    /// `take` should only be called when other references to the slot are
    /// expected to be dropped soon (e.g., when all accesses are relatively
    /// short).
    ///
    /// # Examples
    ///
    /// ```rust
    /// let slab = sharded_slab::Slab::new();
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// // Remove the item from the slab, returning it.
    /// assert_eq!(slab.take(key), Some("hello world"));
    ///
    /// // Now, the slot is empty.
    /// assert!(!slab.contains(key));
    /// ```
    ///
    /// ```rust
    /// use std::sync::Arc;
    ///
    /// let slab = Arc::new(sharded_slab::Slab::new());
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// let slab2 = slab.clone();
    /// let thread2 = std::thread::spawn(move || {
    ///     // Depending on when this thread begins executing, the item may
    ///     // or may not have already been removed...
    ///     if let Some(item) = slab2.get(key) {
    ///         assert_eq!(item, "hello world");
    ///     }
    /// });
    ///
    /// // The item will only be removed when the other thread finishes
    /// // accessing it.
    /// assert_eq!(slab.take(key), Some("hello world"));
    ///
    /// thread2.join().unwrap();
    /// assert!(!slab.contains(key));
    /// ```
    /// [`remove`]: Slab::remove
    pub fn take(&self, idx: usize) -> Option<T> {
        let tid = C::unpack_tid(idx);
        test_println!("rm {:?}", tid);
        let shard = self.shards.get(tid.as_usize())?;
        if tid.is_current() { shard.take_local(idx) } else { shard.take_remote(idx) }
    }
    /// Return a reference to the value associated with the given key.
    ///
    /// If the slab does not contain a value for the given key, or if the
    /// maximum number of concurrent references to the slot has been reached,
    /// `None` is returned instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let slab = sharded_slab::Slab::new();
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// assert_eq!(slab.get(key).unwrap(), "hello world");
    /// assert!(slab.get(12345).is_none());
    /// ```
    pub fn get(&self, key: usize) -> Option<Entry<'_, T, C>> {
        let tid = C::unpack_tid(key);
        test_println!("get {:?}; current={:?}", tid, Tid::< C >::current());
        let shard = self.shards.get(tid.as_usize())?;
        shard
            .with_slot(
                key,
                |slot| {
                    let inner = slot.get(C::unpack_gen(key))?;
                    let value = ptr::NonNull::from(slot.value().as_ref().unwrap());
                    Some(Entry { inner, value, shard, key })
                },
            )
    }
    /// Return an owned reference to the value at the given index.
    ///
    /// If the slab does not contain a value for the given key, `None` is
    /// returned instead.
    ///
    /// Unlike [`get`], which borrows the slab, this method _clones_ the [`Arc`]
    /// around the slab. This means that the returned [`OwnedEntry`] can be held
    /// for an arbitrary lifetime. However,  this method requires that the slab
    /// itself be wrapped in an `Arc`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sharded_slab::Slab;
    /// use std::sync::Arc;
    ///
    /// let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// // Look up the created key, returning an `OwnedEntry`.
    /// let value = slab.clone().get_owned(key).unwrap();
    ///
    /// // Now, the original `Arc` clone of the slab may be dropped, but the
    /// // returned `OwnedEntry` can still access the value.
    /// assert_eq!(value, "hello world");
    /// ```
    ///
    /// Unlike [`Entry`], an `OwnedEntry` may be stored in a struct which must live
    /// for the `'static` lifetime:
    ///
    /// ```
    /// # use sharded_slab::Slab;
    /// use sharded_slab::OwnedEntry;
    /// use std::sync::Arc;
    ///
    /// pub struct MyStruct {
    ///     entry: OwnedEntry<&'static str>,
    ///     // ... other fields ...
    /// }
    ///
    /// // Suppose this is some arbitrary function which requires a value that
    /// // lives for the 'static lifetime...
    /// fn function_requiring_static<T: 'static>(t: &T) {
    ///     // ... do something extremely important and interesting ...
    /// }
    ///
    /// let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// // Look up the created key, returning an `OwnedEntry`.
    /// let entry = slab.clone().get_owned(key).unwrap();
    /// let my_struct = MyStruct {
    ///     entry,
    ///     // ...
    /// };
    ///
    /// // We can use `my_struct` anywhere where it is required to have the
    /// // `'static` lifetime:
    /// function_requiring_static(&my_struct);
    /// ```
    ///
    /// [`OwnedEntry`]s may be sent between threads:
    ///
    /// ```
    /// # use sharded_slab::Slab;
    /// use std::{thread, sync::Arc};
    ///
    /// let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
    /// let key = slab.insert("hello world").unwrap();
    ///
    /// // Look up the created key, returning an `OwnedEntry`.
    /// let value = slab.clone().get_owned(key).unwrap();
    ///
    /// thread::spawn(move || {
    ///     assert_eq!(value, "hello world");
    ///     // ...
    /// }).join().unwrap();
    /// ```
    ///
    /// [`get`]: Slab::get
    /// [`Arc`]: std::sync::Arc
    pub fn get_owned(self: Arc<Self>, key: usize) -> Option<OwnedEntry<T, C>> {
        let tid = C::unpack_tid(key);
        test_println!("get_owned {:?}; current={:?}", tid, Tid::< C >::current());
        let shard = self.shards.get(tid.as_usize())?;
        shard
            .with_slot(
                key,
                |slot| {
                    let inner = slot.get(C::unpack_gen(key))?;
                    let value = ptr::NonNull::from(slot.value().as_ref().unwrap());
                    Some(OwnedEntry {
                        inner,
                        value,
                        slab: self.clone(),
                        key,
                    })
                },
            )
    }
    /// Returns `true` if the slab contains a value for the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// let slab = sharded_slab::Slab::new();
    ///
    /// let key = slab.insert("hello world").unwrap();
    /// assert!(slab.contains(key));
    ///
    /// slab.take(key).unwrap();
    /// assert!(!slab.contains(key));
    /// ```
    pub fn contains(&self, key: usize) -> bool {
        self.get(key).is_some()
    }
    /// Returns an iterator over all the items in the slab.
    ///
    /// Because this iterator exclusively borrows the slab (i.e. it holds an
    /// `&mut Slab<T>`), elements will not be added or removed while the
    /// iteration is in progress.
    pub fn unique_iter(&mut self) -> iter::UniqueIter<'_, T, C> {
        let mut shards = self.shards.iter_mut();
        let (pages, slots) = match shards.next() {
            Some(shard) => {
                let mut pages = shard.iter();
                let slots = pages.next().and_then(page::Shared::iter);
                (pages, slots)
            }
            None => ([].iter(), None),
        };
        iter::UniqueIter {
            shards,
            pages,
            slots,
        }
    }
}
