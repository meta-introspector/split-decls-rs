macro_rules! deps {
    () => {
        Revision!();
    };
}

macro_rules! SlotMemosFnRaw {
    () => {
        deps!();
        # [doc = " [Slot::memos]"] type SlotMemosFnRaw = unsafe fn (* const () , current_revision : Revision) -> * const MemoTable ;
    };
}

SlotMemosFnRaw!()