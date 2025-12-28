macro_rules! not_supported {
    () => {
        fn not_supported < T > (msg : & str) -> io :: Result < T > { Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) }
    };
}

not_supported!()