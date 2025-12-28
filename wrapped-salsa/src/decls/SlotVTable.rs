macro_rules! deps {
    () => {
        Slot!();
        SlotMemosFnRaw!();
        SlotMemosMutFnRaw!();
    };
}

macro_rules! SlotVTable {
    () => {
        deps!();
        struct SlotVTable { layout : Layout , # [doc = " [`Slot`] methods"] memos : SlotMemosFnRaw , memos_mut : SlotMemosMutFnRaw , # [doc = " The type name of what is stored as entries in data."] type_name : fn () -> & 'static str , # [doc = " A drop impl to call when the own page drops"] # [doc = " SAFETY: The caller is required to supply a valid pointer to a `Box<PageDataEntry<T>>`, and"] # [doc = " the correct initialized length and memo types."] drop_impl : unsafe fn (data : * mut () , initialized : usize , memo_types : & MemoTableTypes) , }
    };
}

SlotVTable!();