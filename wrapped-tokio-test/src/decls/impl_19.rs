macro_rules! deps {
    () => {
        StreamMock!();
        Action!();
        StreamMockBuilder!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T : Unpin > StreamMockBuilder < T > { # [doc = " Create a new empty [`StreamMockBuilder`]"] pub fn new () -> Self { StreamMockBuilder :: default () } # [doc = " Queue an item to be returned by the stream"] pub fn next (mut self , value : T) -> Self { self . actions . push_back (Action :: Next (value)) ; self } # [doc = " Queue the stream to wait for a duration"] pub fn wait (mut self , duration : Duration) -> Self { self . actions . push_back (Action :: Wait (duration)) ; self } # [doc = " Build the [`StreamMock`]"] pub fn build (self) -> StreamMock < T > { StreamMock { actions : self . actions , sleep : None , } } }
    };
}

impl_19!()