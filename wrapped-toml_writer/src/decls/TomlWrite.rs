macro_rules! deps {
    () => {
        WriteTomlKey!();
        WriteTomlValue!();
    };
}

macro_rules! TomlWrite {
    () => {
        deps!();
        pub trait TomlWrite : core :: fmt :: Write { fn open_table_header (& mut self) -> core :: fmt :: Result { write ! (self , "[") } fn close_table_header (& mut self) -> core :: fmt :: Result { write ! (self , "]") } fn open_array_of_tables_header (& mut self) -> core :: fmt :: Result { write ! (self , "[[") } fn close_array_of_tables_header (& mut self) -> core :: fmt :: Result { write ! (self , "]]") } fn open_inline_table (& mut self) -> core :: fmt :: Result { write ! (self , "{{") } fn close_inline_table (& mut self) -> core :: fmt :: Result { write ! (self , "}}") } fn open_array (& mut self) -> core :: fmt :: Result { write ! (self , "[") } fn close_array (& mut self) -> core :: fmt :: Result { write ! (self , "]") } fn key_sep (& mut self) -> core :: fmt :: Result { write ! (self , ".") } fn keyval_sep (& mut self) -> core :: fmt :: Result { write ! (self , "=") } # [doc = " Write an encoded TOML key"] # [doc = ""] # [doc = " To customize the encoding, see [`TomlStringBuilder`][crate::TomlStringBuilder]."] fn key (& mut self , value : impl crate :: WriteTomlKey) -> core :: fmt :: Result { value . write_toml_key (self) } # [doc = " Write an encoded TOML scalar value"] # [doc = ""] # [doc = " To customize the encoding, see"] # [doc = " - [`TomlStringBuilder`][crate::TomlStringBuilder]"] # [doc = " - [`TomlIntegerFormat`][crate::TomlIntegerFormat]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " For floats, this preserves the sign bit for [`f32::NAN`] / [`f64::NAN`] for the sake of"] # [doc = " format-preserving editing."] # [doc = " However, in most cases the sign bit is indeterminate and outputting signed NANs can be a"] # [doc = " cause of non-repeatable behavior."] # [doc = ""] # [doc = " For general serialization, you should discard the sign bit.  For example:"] # [doc = " ```"] # [doc = " # let mut v = f64::NAN;"] # [doc = " if v.is_nan() {"] # [doc = "     v = v.copysign(1.0);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " </div>"] fn value (& mut self , value : impl crate :: WriteTomlValue) -> core :: fmt :: Result { value . write_toml_value (self) } fn val_sep (& mut self) -> core :: fmt :: Result { write ! (self , ",") } fn space (& mut self) -> core :: fmt :: Result { write ! (self , " ") } fn open_comment (& mut self) -> core :: fmt :: Result { write ! (self , "#") } fn newline (& mut self) -> core :: fmt :: Result { writeln ! (self) } }
    };
}

TomlWrite!()