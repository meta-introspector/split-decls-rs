macro_rules! deps {
    () => {
        Codetab!();
        CodeQueue!();
    };
}

macro_rules! hwunshrink {
    () => {
        deps!();
        fn hwunshrink (src : & [u8] , uncompressed_size : usize , dst : & mut Vec < u8 >) -> io :: Result < () > { dst . reserve (uncompressed_size) ; let mut codetab = Codetab :: create_new () ; let mut queue = CodeQueue :: new () ; let mut is = BitReader :: endian (src , LittleEndian) ; let mut code_size = MIN_CODE_SIZE ; let Some (curr_code) = read_code (& mut is , & mut code_size , & mut codetab , & mut queue) ? else { return Ok (()) ; } ; debug_assert ! (curr_code != CONTROL_CODE as u16) ; if curr_code > u8 :: MAX as u16 { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "the first code must be a literal" ,)) ; } let mut first_byte = curr_code as u8 ; codetab [curr_code as usize] . last_dst_pos = dst . len () ; dst . push (first_byte) ; let mut prev_code = curr_code ; while dst . len () < uncompressed_size { let curr_opt = match read_code (& mut is , & mut code_size , & mut codetab , & mut queue) { Ok (c) => c , Err (_) => break , } ; let Some (curr_code) = curr_opt else { return Err (Error :: new (io :: ErrorKind :: InvalidData , "invalid code")) ; } ; let dst_pos = dst . len () ; if Some (curr_code) == queue . next () { if codetab [prev_code as usize] . prefix_code . is_none () { return Err (Error :: new (io :: ErrorKind :: InvalidData , "previous code no longer valid" ,)) ; } debug_assert ! (curr_code != prev_code) ; codetab [curr_code as usize] = Codetab { prefix_code : Some (prev_code) , ext_byte : first_byte , len : codetab [prev_code as usize] . len + 1 , last_dst_pos : codetab [prev_code as usize] . last_dst_pos , } ; } first_byte = output_code (dst , curr_code , prev_code , & mut codetab , & queue , first_byte) ? ; if let Some (new_code) = queue . remove_next () { let prev_entry = codetab [prev_code as usize] ; codetab [new_code as usize] = Codetab { prefix_code : Some (prev_code) , ext_byte : first_byte , last_dst_pos : prev_entry . last_dst_pos , len : if prev_entry . prefix_code . is_none () { UNKNOWN_LEN } else { prev_entry . len + 1 } , } ; } codetab [curr_code as usize] . last_dst_pos = dst_pos ; prev_code = curr_code ; } if dst . len () != uncompressed_size { return Err (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "unexpected end of compressed stream" ,)) ; } Ok (()) }
    };
}

hwunshrink!();