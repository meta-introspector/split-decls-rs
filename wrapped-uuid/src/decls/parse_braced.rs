macro_rules! deps {
    () => {
        InvalidUuid!();
    };
}

macro_rules! parse_braced {
    () => {
        deps!();
        # [inline] # [allow (dead_code)] pub (crate) const fn parse_braced (input : & '_ [u8]) -> Result < [u8 ; 16] , InvalidUuid < '_ > > { if let (38 , [b'{' , s @ .. , b'}']) = (input . len () , input) { parse_hyphenated (s) } else { Err (InvalidUuid (input)) } }
    };
}

parse_braced!()