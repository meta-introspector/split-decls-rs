macro_rules! deps {
    () => {
        Slot!();
        Configuration!();
        Value!();
        Revision!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        unsafe impl < C > Slot for Value < C > where C : Configuration , { # [inline (always)] unsafe fn memos (& self , _current_revision : Revision) -> & MemoTable { unsafe { & * self . memos . get () } } # [inline (always)] fn memos_mut (& mut self) -> & mut MemoTable { self . memos . get_mut () } }
    };
}

impl_194!();