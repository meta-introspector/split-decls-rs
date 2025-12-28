macro_rules! deps {
    () => {
        TomlSink!();
        Error!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < 'i > toml_parser :: ErrorSink for TomlSink < 'i , Vec < Error > > { fn report_error (& mut self , error : toml_parser :: ParseError) { let input = self . input . get_or_insert_with (| | alloc :: sync :: Arc :: from (self . source . input ())) ; let error = Error :: new (input . clone () , error) ; self . sink . push (error) ; } }
    };
}

impl_173!();