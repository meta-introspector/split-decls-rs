macro_rules! deps {
    () => {
        ValueSerializer!();
        DocumentMut!();
        Item!();
        Value!();
        Error!();
    };
}

macro_rules! to_document {
    () => {
        deps!();
        # [doc = " Serialize the given data structure into a TOML document."] # [doc = ""] # [doc = " This would allow custom formatting to be applied, mixing with format preserving edits, etc."] pub fn to_document < T > (value : & T) -> Result < crate :: DocumentMut , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let value = value . serialize (ValueSerializer :: new ()) ? ; let item = crate :: Item :: Value (value) ; let root = item . into_table () . map_err (| _ | Error :: UnsupportedType (None)) ? ; Ok (root . into ()) }
    };
}

to_document!()