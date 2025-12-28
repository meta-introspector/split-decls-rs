macro_rules! deps {
    () => {
        Revision!();
        DummySlot!();
        Slot!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        unsafe impl Slot for DummySlot { unsafe fn memos (& self , _ : Revision) -> & MemoTable { unreachable ! () } fn memos_mut (& mut self) -> & mut MemoTable { unreachable ! () } }
    };
}

impl_337!();