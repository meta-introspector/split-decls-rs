macro_rules! deps {
    () => {
        Deserializer!();
        Error!();
        Document!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        # [cfg (feature = "parse")] impl < S : AsRef < str > > Deserializer < S > { # [doc = " Parse a TOML document"] pub fn parse (raw : S) -> Result < Self , Error > { crate :: Document :: parse (raw) . map (Self :: from) . map_err (Into :: into) } }
    };
}

impl_328!()