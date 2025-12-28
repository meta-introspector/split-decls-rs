macro_rules! deps {
    () => {
        Latch!();
    };
}

macro_rules! SET {
    () => {
        deps!();
        # [doc = " Latch is set."] const SET : usize = 3 ;
    };
}

SET!()