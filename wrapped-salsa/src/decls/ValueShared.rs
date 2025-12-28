macro_rules! deps {
    () => {
        Id!();
        Revision!();
        Durability!();
    };
}

macro_rules! ValueShared {
    () => {
        deps!();
        # [doc = " Shared value data can only be read through the lock."] # [repr (Rust , packed)] # [derive (Clone , Copy)] struct ValueShared { # [doc = " The interned ID for this value."] # [doc = ""] # [doc = " Storing this on the value itself is necessary to identify slots"] # [doc = " from the LRU list, as well as keep track of the generation."] # [doc = ""] # [doc = " Values that are reused increment the ID generation, as if they had"] # [doc = " allocated a new slot. This eliminates the need for dependency edges"] # [doc = " on queries that *read* from an interned value, as any memos dependent"] # [doc = " on the previous value will not match the new ID."] # [doc = ""] # [doc = " However, reusing a slot invalidates the previous ID, so dependency edges"] # [doc = " on queries that *create* an interned value are still required to ensure"] # [doc = " the value is re-interned with a new ID."] id : Id , # [doc = " The revision the value was most-recently interned in."] last_interned_at : Revision , # [doc = " The minimum durability of all inputs consumed by the creator"] # [doc = " query prior to creating this interned struct. If any of those"] # [doc = " inputs changes, then the creator query may create this struct"] # [doc = " with different values."] durability : Durability , }
    };
}

ValueShared!()