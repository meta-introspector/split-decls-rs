macro_rules! deps {
    () => {
        TomlSink!();
        TomlError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < 'i > toml_parser :: ErrorSink for TomlSink < 'i , Vec < TomlError > > { fn report_error (& mut self , error : toml_parser :: ParseError) { let input = self . input . get_or_insert_with (| | std :: sync :: Arc :: from (self . source . input ())) ; let error = TomlError :: new (input . clone () , error) ; self . sink . push (error) ; } }
    };
}

impl_72!();