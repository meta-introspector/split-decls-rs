macro_rules! SlotIndex {
    () => {
        # [doc = " This uniquely identifies a single `Slot<V>` entry in the buckets map, and provides accessors for"] # [doc = " either getting the value or putting a value."] # [derive (Copy , Clone , Debug)] struct SlotIndex { bucket_idx : usize , entries : usize , index_in_bucket : usize , }
    };
}

SlotIndex!()