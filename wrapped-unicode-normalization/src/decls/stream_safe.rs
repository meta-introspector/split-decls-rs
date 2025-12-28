macro_rules! deps {
    () => {
        StreamSafe!();
    };
}

macro_rules! stream_safe {
    () => {
        deps!();
        pub fn stream_safe (s : & str) -> String { StreamSafe :: new (s . chars ()) . collect () }
    };
}

stream_safe!()