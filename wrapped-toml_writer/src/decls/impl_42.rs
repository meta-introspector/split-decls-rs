macro_rules! deps {
    () => {
        ValueMetrics!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl ValueMetrics { fn new () -> Self { Self { max_seq_single_quotes : 0 , max_seq_double_quotes : 0 , escape_codes : false , escape : false , newline : false , } } fn calculate (s : & str) -> Self { let mut metrics = Self :: new () ; let mut prev_single_quotes = 0 ; let mut prev_double_quotes = 0 ; for byte in s . as_bytes () { if * byte == b'\'' { prev_single_quotes += 1 ; metrics . max_seq_single_quotes = metrics . max_seq_single_quotes . max (prev_single_quotes) ; } else { prev_single_quotes = 0 ; } if * byte == b'"' { prev_double_quotes += 1 ; metrics . max_seq_double_quotes = metrics . max_seq_double_quotes . max (prev_double_quotes) ; } else { prev_double_quotes = 0 ; } match * byte { b'\\' => metrics . escape = true , b'\t' => { } b'\n' => metrics . newline = true , c if c <= 0x1f || c == 0x7f => metrics . escape_codes = true , _ => { } } } metrics } }
    };
}

impl_42!();