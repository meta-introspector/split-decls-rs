macro_rules! deps {
    () => {
        ValueDeserializer!();
        Error!();
    };
}

macro_rules! from_slice {
    () => {
        deps!();
        # [doc = " Deserializes bytes into a type."] # [doc = ""] # [doc = " This function will attempt to interpret `s` as a TOML document and"] # [doc = " deserialize `T` from the document."] # [doc = ""] # [doc = " To deserializes TOML values, instead of documents, see [`ValueDeserializer`]."] # [cfg (feature = "parse")] # [cfg (feature = "serde")] pub fn from_slice < 'de , T > (s : & 'de [u8]) -> Result < T , Error > where T : serde_core :: de :: Deserialize < 'de > , { let s = core :: str :: from_utf8 (s) . map_err (| e | Error :: custom (e . to_string () , None)) ? ; from_str (s) }
    };
}

from_slice!();