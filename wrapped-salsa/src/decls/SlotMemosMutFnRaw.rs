macro_rules! SlotMemosMutFnRaw {
    () => {
        # [doc = " [Slot::memos_mut]"] type SlotMemosMutFnRaw = unsafe fn (* mut ()) -> * mut MemoTable ;
    };
}

SlotMemosMutFnRaw!();