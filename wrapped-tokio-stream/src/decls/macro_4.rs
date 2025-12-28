macro_rules! macro_4 {
    () => {
        cfg_time ! { # [deprecated = "Import those symbols from adapters instead"] # [doc (hidden)] pub use stream_ext :: timeout :: Timeout ; pub use stream_ext :: timeout :: Elapsed ; }
    };
}

macro_4!()