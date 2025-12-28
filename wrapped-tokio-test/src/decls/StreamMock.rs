macro_rules! deps {
    () => {
        StreamMockBuilder!();
        Action!();
    };
}

macro_rules! StreamMock {
    () => {
        deps!();
        # [doc = " A mock stream implementing [`Stream`]"] # [doc = ""] # [doc = " See [`StreamMockBuilder`] for more information."] # [derive (Debug)] pub struct StreamMock < T : Unpin > { actions : VecDeque < Action < T > > , sleep : Option < Pin < Box < Sleep > > > , }
    };
}

StreamMock!()