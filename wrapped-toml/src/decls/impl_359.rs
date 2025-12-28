macro_rules! deps {
    () => {
        Style!();
        ValueSerializer!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < 'd > ValueSerializer < 'd > { # [doc = " Creates a new serializer which will emit TOML into the buffer provided."] # [doc = ""] # [doc = " The serializer can then be used to serialize a type after which the data"] # [doc = " will be present in `dst`."] pub fn new (dst : & 'd mut String) -> Self { Self { dst , style : Default :: default () , } } pub (crate) fn with_style (dst : & 'd mut String , style : Style) -> Self { Self { dst , style } } }
    };
}

impl_359!();