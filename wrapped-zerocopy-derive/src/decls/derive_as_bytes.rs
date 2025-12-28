macro_rules! derive_as_bytes {
    () => {
        # [doc = " Deprecated: prefer [`IntoBytes`] instead."] # [deprecated (since = "0.8.0" , note = "`AsBytes` was renamed to `IntoBytes`")] # [doc (hidden)] # [proc_macro_derive (AsBytes)] pub fn derive_as_bytes (ts : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_into_bytes (ts) }
    };
}

derive_as_bytes!()