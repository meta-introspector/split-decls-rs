macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl AsRef < std :: path :: Path > for SmolStr { # [inline (always)] fn as_ref (& self) -> & std :: path :: Path { AsRef :: < std :: path :: Path > :: as_ref (self . as_str ()) } }
    };
}

impl_31!();