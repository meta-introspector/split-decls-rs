macro_rules! deps {
    () => {
        SlotIndex!();
        PageIndex!();
        Id!();
    };
}

macro_rules! make_id {
    () => {
        deps!();
        fn make_id (page : PageIndex , slot : SlotIndex) -> Id { let page = page . 0 as u32 ; let slot = slot . 0 as u32 ; unsafe { Id :: from_index ((page << PAGE_LEN_BITS) | slot) } }
    };
}

make_id!();