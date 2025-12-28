macro_rules! deps {
    () => {
        State!();
        Mode!();
    };
}

macro_rules! set_mode_dict {
    () => {
        deps!();
        # [cfg (feature = "__internal-test")] # [doc (hidden)] pub unsafe fn set_mode_dict (strm : & mut z_stream) { unsafe { (* (strm . state as * mut State)) . mode = Mode :: Dict ; } }
    };
}

set_mode_dict!();