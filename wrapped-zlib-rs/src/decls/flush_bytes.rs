macro_rules! deps {
    () => {
        ReturnCode!();
        DeflateStream!();
    };
}

macro_rules! flush_bytes {
    () => {
        deps!();
        fn flush_bytes (stream : & mut DeflateStream , mut bytes : & [u8]) -> ControlFlow < ReturnCode > { let mut state = & mut stream . state ; let mut beg = state . bit_writer . pending . pending () . len () ; while state . bit_writer . pending . remaining () < bytes . len () { let copy = state . bit_writer . pending . remaining () ; state . bit_writer . pending . extend (& bytes [.. copy]) ; stream . adler = crc32 (stream . adler as u32 , & state . bit_writer . pending . pending () [beg ..] ,) as z_checksum ; state . gzindex += copy ; flush_pending (stream) ; state = & mut stream . state ; if ! state . bit_writer . pending . pending () . is_empty () { state . last_flush = - 1 ; return ControlFlow :: Break (ReturnCode :: Ok) ; } beg = 0 ; bytes = & bytes [copy ..] ; } state . bit_writer . pending . extend (bytes) ; stream . adler = crc32 (stream . adler as u32 , & state . bit_writer . pending . pending () [beg ..] ,) as z_checksum ; state . gzindex = 0 ; ControlFlow :: Continue (()) }
    };
}

flush_bytes!();