macro_rules! TomlError {
    () => {
        # [doc = " A TOML parse error"] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub struct TomlError { message : String , input : Option < std :: sync :: Arc < str > > , keys : Vec < String > , span : Option < std :: ops :: Range < usize > > , }
    };
}

TomlError!()