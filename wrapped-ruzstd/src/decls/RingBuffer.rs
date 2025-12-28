macro_rules! RingBuffer {
    () => {
        pub struct RingBuffer { buf : NonNull < u8 > , cap : usize , head : usize , tail : usize , }
    };
}

RingBuffer!();