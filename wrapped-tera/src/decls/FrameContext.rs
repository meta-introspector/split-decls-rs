macro_rules! deps {
    () => {
        Val!();
    };
}

macro_rules! FrameContext {
    () => {
        deps!();
        pub type FrameContext < 'a > = HashMap < & 'a str , Val < 'a > > ;
    };
}

FrameContext!()