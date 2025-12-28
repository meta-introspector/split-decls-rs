macro_rules! deps {
    () => {
        StreamMock!();
        Action!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : Unpin > StreamMock < T > { fn next_action (& mut self) -> Option < Action < T > > { self . actions . pop_front () } }
    };
}

impl_22!()