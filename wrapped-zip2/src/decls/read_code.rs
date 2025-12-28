macro_rules! deps {
    () => {
        CodeQueue!();
        Codetab!();
    };
}

macro_rules! read_code {
    () => {
        deps!();
        # [doc = " Read the next code from the input stream and return it in next_code. Returns"] # [doc = " `Ok(None)` if the end of the stream is reached. If the stream contains invalid"] # [doc = " data, returns `Err`."] fn read_code < T : std :: io :: Read , E : Endianness > (is : & mut BitReader < T , E > , code_size : & mut u8 , codetab : & mut [Codetab] , queue : & mut CodeQueue ,) -> io :: Result < Option < u16 > > { let code = is . read_var :: < u16 > (* code_size as u32) ? ; if code != CONTROL_CODE as u16 { return Ok (Some (code)) ; } if let Ok (control_code) = is . read_var :: < u16 > (* code_size as u32) { match control_code { INC_CODE_SIZE => { if * code_size >= MAX_CODE_SIZE { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "tried to increase code size when already at maximum" ,)) ; } * code_size += 1 ; } PARTIAL_CLEAR => { unshrink_partial_clear (codetab , queue) ; } _ => { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , format ! ("Invalid control code {}" , control_code) ,)) ; } } return read_code (is , code_size , codetab , queue) ; } Ok (None) }
    };
}

read_code!()