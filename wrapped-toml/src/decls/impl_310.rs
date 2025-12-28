macro_rules! deps {
    () => {
        Table!();
        Serializer!();
        Style!();
        Buffer!();
        Error!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'd > Serializer < 'd > { # [doc = " Creates a new serializer which will emit TOML into the buffer provided."] # [doc = ""] # [doc = " The serializer can then be used to serialize a type after which the data"] # [doc = " will be present in `buf`."] pub fn new (buf : & 'd mut Buffer) -> Self { let table = buf . root_table () ; Self { buf , style : Default :: default () , table , } } # [doc = " Apply a default \"pretty\" policy to the document"] # [doc = ""] # [doc = " For greater customization, instead serialize to a"] # [doc = " [`toml_edit::DocumentMut`](https://docs.rs/toml_edit/latest/toml_edit/struct.DocumentMut.html)."] pub fn pretty (buf : & 'd mut Buffer) -> Self { let mut ser = Serializer :: new (buf) ; ser . style . multiline_array = true ; ser } pub (crate) fn with_table (buf : & 'd mut Buffer , table : Table , style : style :: Style) -> Self { Self { buf , style , table } } fn end (self) -> Result < & 'd mut Buffer , Error > { self . buf . push (self . table) ; Ok (self . buf) } }
    };
}

impl_310!();