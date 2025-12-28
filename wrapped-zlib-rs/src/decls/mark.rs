macro_rules! deps {
    () => {
        InflateStream!();
        Mode!();
    };
}

macro_rules! mark {
    () => {
        deps!();
        pub fn mark (stream : & InflateStream) -> c_long { if stream . next_out . is_null () || (stream . next_in . is_null () && stream . avail_in != 0) { return c_long :: MIN ; } let state = & stream . state ; let length = match state . mode { Mode :: CopyBlock => state . length , Mode :: Match => state . was - state . length , _ => 0 , } ; (((state . back as c_long) as c_ulong) << 16) as c_long + length as c_long }
    };
}

mark!()