macro_rules! macro_2 {
    () => {
        cfg_codec ! { # [macro_use] mod tracing ; pub mod codec ; }
    };
}

macro_2!()