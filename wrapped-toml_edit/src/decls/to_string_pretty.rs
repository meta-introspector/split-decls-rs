macro_rules! deps {
    () => {
        Pretty!();
        Error!();
    };
}

macro_rules! to_string_pretty {
    () => {
        deps!();
        # [doc = " Serialize the given data structure as a \"pretty\" String of TOML."] # [doc = ""] # [doc = " This is identical to `to_string` except the output string has a more"] # [doc = " \"pretty\" output. See `ValueSerializer::pretty` for more details."] # [cfg (feature = "display")] pub fn to_string_pretty < T > (value : & T) -> Result < String , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let mut document = to_document (value) ? ; pretty :: Pretty :: new () . visit_document_mut (& mut document) ; Ok (document . to_string ()) }
    };
}

to_string_pretty!()