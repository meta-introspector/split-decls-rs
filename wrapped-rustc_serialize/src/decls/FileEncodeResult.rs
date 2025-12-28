macro_rules! FileEncodeResult {
    () => {
        pub type FileEncodeResult = Result < usize , (PathBuf , io :: Error) > ;
    };
}

FileEncodeResult!()