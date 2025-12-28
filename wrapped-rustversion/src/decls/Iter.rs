macro_rules! deps {
    () => {
        IterImpl!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        pub type Iter < 'a > = & 'a mut IterImpl ;
    };
}

Iter!()