macro_rules! set_deref {
    () => {
        # [doc = " Set an extended attribute on the specified file (dereference symlinks)."] pub fn set_deref < N , P > (path : P , name : N , value : & [u8]) -> io :: Result < () > where P : AsRef < Path > , N : AsRef < OsStr > , { sys :: set_path (path . as_ref () , name . as_ref () , value , true) }
    };
}

set_deref!()