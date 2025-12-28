macro_rules! remove_deref {
    () => {
        # [doc = " Remove an extended attribute from the specified file (dereference symlinks)."] pub fn remove_deref < N , P > (path : P , name : N) -> io :: Result < () > where P : AsRef < Path > , N : AsRef < OsStr > , { sys :: remove_path (path . as_ref () , name . as_ref () , true) }
    };
}

remove_deref!()