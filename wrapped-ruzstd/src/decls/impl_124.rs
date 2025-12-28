macro_rules! deps {
    () => {
        FrameDecoder!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Default for FrameDecoder { fn default () -> Self { Self :: new () } }
    };
}

impl_124!();