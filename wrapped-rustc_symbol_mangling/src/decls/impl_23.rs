macro_rules! deps {
    () => {
        LegacySymbolMangler!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl fmt :: Write for LegacySymbolMangler < '_ > { fn write_str (& mut self , s : & str) -> fmt :: Result { for c in s . chars () { if self . path . temp_buf . is_empty () { match c { 'a' ..= 'z' | 'A' ..= 'Z' | '_' => { } _ => { self . path . temp_buf . push ('_') ; } } } match c { '@' => self . path . temp_buf . push_str ("$SP$") , '*' => self . path . temp_buf . push_str ("$BP$") , '&' => self . path . temp_buf . push_str ("$RF$") , '<' => self . path . temp_buf . push_str ("$LT$") , '>' => self . path . temp_buf . push_str ("$GT$") , '(' => self . path . temp_buf . push_str ("$LP$") , ')' => self . path . temp_buf . push_str ("$RP$") , ',' => self . path . temp_buf . push_str ("$C$") , '-' | ':' | '.' if self . tcx . has_strict_asm_symbol_naming () => { self . path . temp_buf . push ('$') } '-' | ':' => self . path . temp_buf . push ('.') , 'm' if self . path . temp_buf . ends_with (".llv") => self . path . temp_buf . push_str ("$u6d$") , 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' | '.' | '$' => self . path . temp_buf . push (c) , _ => { self . path . temp_buf . push ('$') ; for c in c . escape_unicode () . skip (1) { match c { '{' => { } '}' => self . path . temp_buf . push ('$') , c => self . path . temp_buf . push (c) , } } } } } Ok (()) } }
    };
}

impl_23!();