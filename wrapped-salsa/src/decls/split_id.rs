macro_rules! deps {
    () => {
        PageIndex!();
        Id!();
        SlotIndex!();
    };
}

macro_rules! split_id {
    () => {
        deps!();
        # [inline] pub fn split_id (id : Id) -> (PageIndex , SlotIndex) { let index = id . index () as usize ; let slot = index & PAGE_LEN_MASK ; let page = index >> PAGE_LEN_BITS ; (PageIndex :: new (page) , SlotIndex :: new (slot)) }
    };
}

split_id!()