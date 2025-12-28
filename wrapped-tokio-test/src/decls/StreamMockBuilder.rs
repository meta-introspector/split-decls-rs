macro_rules! deps {
    () => {
        Action!();
        StreamMock!();
    };
}

macro_rules! StreamMockBuilder {
    () => {
        deps!();
        # [doc = " A builder for [`StreamMock`]"] # [derive (Debug , Clone)] pub struct StreamMockBuilder < T : Unpin > { actions : VecDeque < Action < T > > , }
    };
}

StreamMockBuilder!();