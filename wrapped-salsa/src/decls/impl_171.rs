macro_rules! deps {
    () => {
        Configuration!();
        Slot!();
        Value!();
        Revision!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        unsafe impl < C > Slot for Value < C > where C : Configuration , { # [inline (always)] unsafe fn memos (& self , _current_revision : Revision) -> & crate :: table :: memo :: MemoTable { & self . memos } # [inline (always)] fn memos_mut (& mut self) -> & mut crate :: table :: memo :: MemoTable { & mut self . memos } }
    };
}

impl_171!();