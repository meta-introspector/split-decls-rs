macro_rules! macro_340 {
    () => {
        cfg_rt ! { pub (crate) struct NotSendOrSync (# [allow (dead_code)] * mut ()) ; }
    };
}

macro_340!()