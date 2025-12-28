macro_rules! WakeList {
    () => {
        # [doc = " A list of wakers to be woken."] # [doc = ""] # [doc = " # Invariants"] # [doc = ""] # [doc = " The first `curr` elements of `inner` are initialized."] pub (crate) struct WakeList { inner : [MaybeUninit < Waker > ; NUM_WAKERS] , curr : usize , }
    };
}

WakeList!();