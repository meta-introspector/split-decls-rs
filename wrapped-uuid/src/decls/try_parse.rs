macro_rules! deps {
    () => {
        InvalidUuid!();
    };
}

macro_rules! try_parse {
    () => {
        deps!();
        const fn try_parse (input : & '_ [u8]) -> Result < [u8 ; 16] , InvalidUuid < '_ > > { match (input . len () , input) { (32 , s) => parse_simple (s) , (36 , s) | (38 , [b'{' , s @ .. , b'}']) | (45 , [b'u' , b'r' , b'n' , b':' , b'u' , b'u' , b'i' , b'd' , b':' , s @ ..]) => { parse_hyphenated (s) } _ => Err (InvalidUuid (input)) , } }
    };
}

try_parse!()