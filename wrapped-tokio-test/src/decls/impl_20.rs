macro_rules! deps {
    () => {
        StreamMockBuilder!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : Unpin > Default for StreamMockBuilder < T > { fn default () -> Self { StreamMockBuilder { actions : VecDeque :: new () , } } }
    };
}

impl_20!()