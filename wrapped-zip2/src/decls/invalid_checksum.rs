macro_rules! invalid_checksum {
    () => {
        # [cold] fn invalid_checksum () -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , "Invalid checksum") }
    };
}

invalid_checksum!()