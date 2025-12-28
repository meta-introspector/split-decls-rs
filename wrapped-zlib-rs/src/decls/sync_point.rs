macro_rules! deps {
    () => {
        Mode!();
        InflateStream!();
    };
}

macro_rules! sync_point {
    () => {
        deps!();
        pub fn sync_point (stream : & mut InflateStream) -> bool { matches ! (stream . state . mode , Mode :: Stored) && stream . state . bit_reader . bits_in_buffer () == 0 }
    };
}

sync_point!();