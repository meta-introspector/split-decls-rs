macro_rules! deps {
    () => {
        ValueMetrics!();
        TomlStringBuilder!();
        TomlString!();
        Encoding!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 's > TomlStringBuilder < 's > { pub fn new (decoded : & 's str) -> Self { Self { decoded , metrics : ValueMetrics :: calculate (decoded) , } } pub fn as_default (& self) -> TomlString < 's > { self . as_basic_pretty () . or_else (| | self . as_literal ()) . or_else (| | self . as_ml_basic_pretty ()) . or_else (| | self . as_ml_literal ()) . unwrap_or_else (| | { if self . metrics . newline { self . as_ml_basic () } else { self . as_basic () } }) } pub fn as_literal (& self) -> Option < TomlString < 's > > { if self . metrics . escape_codes || 0 < self . metrics . max_seq_single_quotes || self . metrics . newline { None } else { Some (TomlString { decoded : self . decoded , encoding : Encoding :: LiteralString , newline : self . metrics . newline , }) } } pub fn as_ml_literal (& self) -> Option < TomlString < 's > > { if self . metrics . escape_codes || 2 < self . metrics . max_seq_single_quotes { None } else { Some (TomlString { decoded : self . decoded , encoding : Encoding :: MlLiteralString , newline : self . metrics . newline , }) } } pub fn as_basic_pretty (& self) -> Option < TomlString < 's > > { if self . metrics . escape_codes || self . metrics . escape || 0 < self . metrics . max_seq_double_quotes || self . metrics . newline { None } else { Some (self . as_basic ()) } } pub fn as_ml_basic_pretty (& self) -> Option < TomlString < 's > > { if self . metrics . escape_codes || self . metrics . escape || 2 < self . metrics . max_seq_double_quotes { None } else { Some (self . as_ml_basic ()) } } pub fn as_basic (& self) -> TomlString < 's > { TomlString { decoded : self . decoded , encoding : Encoding :: BasicString , newline : self . metrics . newline , } } pub fn as_ml_basic (& self) -> TomlString < 's > { TomlString { decoded : self . decoded , encoding : Encoding :: MlBasicString , newline : self . metrics . newline , } } }
    };
}

impl_31!();