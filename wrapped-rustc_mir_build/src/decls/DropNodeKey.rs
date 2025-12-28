macro_rules! deps {
    () => {
        DropNode!();
    };
}

macro_rules! DropNodeKey {
    () => {
        deps!();
        # [doc = " Subset of [`DropNode`] used for reverse lookup in a hash table."] # [derive (Debug , PartialEq , Eq , Hash)] struct DropNodeKey { next : DropIdx , local : Local , }
    };
}

DropNodeKey!();