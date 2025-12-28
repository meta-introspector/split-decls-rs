macro_rules! TomlSink {
    () => {
        # [cfg (feature = "parse")] pub (crate) struct TomlSink < 'i , S > { source : toml_parser :: Source < 'i > , input : Option < alloc :: sync :: Arc < str > > , sink : S , }
    };
}

TomlSink!();