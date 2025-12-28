macro_rules! deps {
    () => {
        Revision!();
    };
}

macro_rules! SlotMemosFn {
    () => {
        deps!();
        # [doc = " [Slot::memos]"] type SlotMemosFn < T > = unsafe fn (& T , current_revision : Revision) -> & MemoTable ;
    };
}

SlotMemosFn!()