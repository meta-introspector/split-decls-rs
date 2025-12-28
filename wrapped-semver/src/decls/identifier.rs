macro_rules! deps {
    () => {
        ErrorKind!();
        Position!();
        Error!();
    };
}

macro_rules! identifier {
    () => {
        deps!();
        fn identifier (input : & str , pos : Position) -> Result < (& str , & str) , Error > { let mut accumulated_len = 0 ; let mut segment_len = 0 ; let mut segment_has_nondigit = false ; loop { match input . as_bytes () . get (accumulated_len + segment_len) { Some (b'A' ..= b'Z') | Some (b'a' ..= b'z') | Some (b'-') => { segment_len += 1 ; segment_has_nondigit = true ; } Some (b'0' ..= b'9') => { segment_len += 1 ; } boundary => { if segment_len == 0 { if accumulated_len == 0 && boundary != Some (& b'.') { return Ok (("" , input)) ; } else { return Err (Error :: new (ErrorKind :: EmptySegment (pos))) ; } } if pos == Position :: Pre && segment_len > 1 && ! segment_has_nondigit && input [accumulated_len ..] . starts_with ('0') { return Err (Error :: new (ErrorKind :: LeadingZero (pos))) ; } accumulated_len += segment_len ; if boundary == Some (& b'.') { accumulated_len += 1 ; segment_len = 0 ; segment_has_nondigit = false ; } else { return Ok (input . split_at (accumulated_len)) ; } } } } }
    };
}

identifier!()