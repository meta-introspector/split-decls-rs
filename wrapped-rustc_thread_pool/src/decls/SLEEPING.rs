macro_rules! deps {
    () => {
        Latch!();
    };
}

macro_rules! SLEEPING {
    () => {
        deps!();
        # [doc = " Latch is not set, owning thread is asleep on this latch and"] # [doc = " must be awoken."] const SLEEPING : usize = 2 ;
    };
}

SLEEPING!()