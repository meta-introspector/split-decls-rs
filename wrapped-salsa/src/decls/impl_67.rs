macro_rules! deps {
    () => {
        CycleHeadIdsIterator!();
        Id!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Iterator for CycleHeadIdsIterator < '_ > { type Item = crate :: Id ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| head | head . database_key_index . key_index ()) } }
    };
}

impl_67!();