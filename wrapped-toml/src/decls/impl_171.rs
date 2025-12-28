macro_rules! deps {
    () => {
        TomlSink!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < 'i , S : Default > TomlSink < 'i , S > { pub (crate) fn new (source : toml_parser :: Source < 'i >) -> Self { Self { source , input : None , sink : Default :: default () , } } pub (crate) fn into_inner (self) -> S { self . sink } }
    };
}

impl_171!()