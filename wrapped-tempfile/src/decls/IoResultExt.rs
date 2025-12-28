macro_rules! IoResultExt {
    () => {
        pub (crate) trait IoResultExt < T > { fn with_err_path < F , P > (self , path : F) -> Self where F : FnOnce () -> P , P : Into < PathBuf > ; }
    };
}

IoResultExt!()