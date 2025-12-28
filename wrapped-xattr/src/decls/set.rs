macro_rules! set {
    () => {
        # [doc = " Set an extended attribute on the specified file."] pub fn set < N , P > (path : P , name : N , value : & [u8]) -> io :: Result < () > where P : AsRef < Path > , N : AsRef < OsStr > , { sys :: set_path (path . as_ref () , name . as_ref () , value , false) }
    };
}

set!();