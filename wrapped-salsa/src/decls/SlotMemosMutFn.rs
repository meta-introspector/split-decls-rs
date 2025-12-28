macro_rules! SlotMemosMutFn {
    () => {
        # [doc = " [Slot::memos_mut]"] type SlotMemosMutFn < T > = fn (& mut T) -> & mut MemoTable ;
    };
}

SlotMemosMutFn!()