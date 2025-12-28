macro_rules! deps {
    () => {
        CodeQueue!();
        Codetab!();
    };
}

macro_rules! output_code {
    () => {
        deps!();
        # [doc = " Output the string represented by a code into dst at dst_pos."] # [doc = ""] # [doc = " # Returns"] # [doc = " new first byte of the string, or io::Error InvalidData on invalid prefix code."] fn output_code (dst : & mut Vec < u8 > , code : u16 , prev_code : u16 , codetab : & mut [Codetab] , queue : & CodeQueue , first_byte : u8 ,) -> io :: Result < u8 > { debug_assert ! (code <= MAX_CODE as u16 && code != CONTROL_CODE as u16) ; if code <= u8 :: MAX as u16 { dst . push (code as u8) ; return Ok (code as u8) ; } if codetab [code as usize] . prefix_code . is_none () || codetab [code as usize] . prefix_code == Some (code) { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "invalid code")) ; } if codetab [code as usize] . len != UNKNOWN_LEN { let ct = & codetab [code as usize] ; for i in ct . last_dst_pos .. ct . last_dst_pos + ct . len as usize { dst . push (dst [i]) ; } return Ok (dst [ct . last_dst_pos]) ; } let prefix_code = codetab [code as usize] . prefix_code . unwrap () ; if cfg ! (debug_assertions) { let tab_entry = codetab [code as usize] ; assert ! (tab_entry . len == UNKNOWN_LEN) ; assert ! (tab_entry . prefix_code . unwrap () as usize > CONTROL_CODE) ; } if Some (prefix_code) == queue . next () { codetab [prefix_code as usize] = Codetab { prefix_code : Some (prev_code) , ext_byte : first_byte , len : codetab [prev_code as usize] . len + 1 , last_dst_pos : codetab [prev_code as usize] . last_dst_pos , } ; dst . push (first_byte) ; } else if codetab [prefix_code as usize] . prefix_code . is_none () { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "invalid prefix code" ,)) ; } let ct = codetab [prefix_code as usize] ; let last_dst_pos = dst . len () ; let start = ct . last_dst_pos ; let len = ct . len as usize ; let first = dst [start] ; dst . extend_from_within (start .. start + len) ; dst . push (codetab [code as usize] . ext_byte) ; debug_assert ! (prev_code != code) ; codetab [code as usize] . len = (len + 1) as u16 ; codetab [code as usize] . last_dst_pos = last_dst_pos ; Ok (first) }
    };
}

output_code!()