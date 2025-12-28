macro_rules! error {
    () => {
        fn error < T > () -> io :: Result < T > { Err (io :: Error :: new (io :: ErrorKind :: Other , ERROR_MESSAGE)) }
    };
}

error!()