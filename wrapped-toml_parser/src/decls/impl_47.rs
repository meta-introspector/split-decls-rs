macro_rules! deps {
    () => {
        IntegerRadix!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl IntegerRadix { pub fn description (& self) -> & 'static str { match self { Self :: Dec => "integer" , Self :: Hex => "hexadecimal" , Self :: Oct => "octal" , Self :: Bin => "binary" , } } pub fn value (& self) -> u32 { match self { Self :: Dec => 10 , Self :: Hex => 16 , Self :: Oct => 8 , Self :: Bin => 2 , } } pub fn invalid_description (& self) -> & 'static str { match self { Self :: Dec => "invalid integer number" , Self :: Hex => "invalid hexadecimal number" , Self :: Oct => "invalid octal number" , Self :: Bin => "invalid binary number" , } } fn validator (& self) -> fn (char) -> bool { match self { Self :: Dec => | c | c . is_ascii_digit () , Self :: Hex => | c | c . is_ascii_hexdigit () , Self :: Oct => | c | matches ! (c , '0' ..='7') , Self :: Bin => | c | matches ! (c , '0' ..='1') , } } }
    };
}

impl_47!()