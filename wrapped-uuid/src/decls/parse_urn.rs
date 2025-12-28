macro_rules! deps {
    () => {
        InvalidUuid!();
    };
}

macro_rules! parse_urn {
    () => {
        deps!();
        # [inline] # [allow (dead_code)] pub (crate) const fn parse_urn (input : & '_ [u8]) -> Result < [u8 ; 16] , InvalidUuid < '_ > > { if let (45 , [b'u' , b'r' , b'n' , b':' , b'u' , b'u' , b'i' , b'd' , b':' , s @ ..]) = (input . len () , input) { parse_hyphenated (s) } else { Err (InvalidUuid (input)) } }
    };
}

parse_urn!();