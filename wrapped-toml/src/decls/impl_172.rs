macro_rules! deps {
    () => {
        TomlSink!();
        Error!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < 'i > toml_parser :: ErrorSink for TomlSink < 'i , Option < Error > > { fn report_error (& mut self , error : toml_parser :: ParseError) { if self . sink . is_none () { let input = self . input . get_or_insert_with (| | alloc :: sync :: Arc :: from (self . source . input ())) ; let error = Error :: new (input . clone () , error) ; self . sink = Some (error) ; } } }
    };
}

impl_172!()