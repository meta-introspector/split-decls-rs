macro_rules! deps {
    () => {
        BitWriter!();
    };
}

macro_rules! encode_seqnum {
    () => {
        deps!();
        fn encode_seqnum (seqnum : usize , writer : & mut BitWriter < impl AsMut < Vec < u8 > > >) { const UPPER_LIMIT : usize = 0xFFFF + 0x7F00 ; match seqnum { 1 ..= 127 => writer . write_bits (seqnum as u32 , 8) , 128 ..= 0x7FFF => { let upper = ((seqnum >> 8) | 0x80) as u8 ; let lower = seqnum as u8 ; writer . write_bits (upper , 8) ; writer . write_bits (lower , 8) ; } 0x8000 ..= UPPER_LIMIT => { let encode = seqnum - 0x7F00 ; let upper = (encode >> 8) as u8 ; let lower = encode as u8 ; writer . write_bits (255u8 , 8) ; writer . write_bits (upper , 8) ; writer . write_bits (lower , 8) ; } _ => unreachable ! () , } }
    };
}

encode_seqnum!();