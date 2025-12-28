macro_rules! get {
    () => {
        # [doc = " Get an extended attribute for the specified file."] pub fn get < N , P > (path : P , name : N) -> io :: Result < Option < Vec < u8 > > > where P : AsRef < Path > , N : AsRef < OsStr > , { util :: extract_noattr (sys :: get_path (path . as_ref () , name . as_ref () , false)) }
    };
}

get!();