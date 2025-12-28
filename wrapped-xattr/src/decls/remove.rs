macro_rules! remove {
    () => {
        # [doc = " Remove an extended attribute from the specified file."] pub fn remove < N , P > (path : P , name : N) -> io :: Result < () > where P : AsRef < Path > , N : AsRef < OsStr > , { sys :: remove_path (path . as_ref () , name . as_ref () , false) }
    };
}

remove!();