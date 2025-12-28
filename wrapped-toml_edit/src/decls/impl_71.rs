macro_rules! deps {
    () => {
        TomlError!();
        TomlSink!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < 'i > toml_parser :: ErrorSink for TomlSink < 'i , Option < TomlError > > { fn report_error (& mut self , error : toml_parser :: ParseError) { if self . sink . is_none () { let input = self . input . get_or_insert_with (| | std :: sync :: Arc :: from (self . source . input ())) ; let error = TomlError :: new (input . clone () , error) ; self . sink = Some (error) ; } } }
    };
}

impl_71!();