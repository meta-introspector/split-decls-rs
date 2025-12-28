macro_rules! deps {
    () => {
        ArrayOfTablesSerializer!();
        Buffer!();
        Table!();
        Style!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'd > ArrayOfTablesSerializer < 'd > { # [doc = " Creates a new serializer which will emit TOML into the buffer provided."] # [doc = ""] # [doc = " The serializer can then be used to serialize a type after which the data"] # [doc = " will be present in `dst`."] pub (crate) fn new (buf : & 'd mut Buffer , parent : Table , key : String , style : Style) -> Self { Self { buf , parent , key , style , } } }
    };
}

impl_270!();