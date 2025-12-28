macro_rules! deps {
    () => {
        Lookup!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        # [cfg (feature = "compact_str")] impl Lookup < compact_str :: CompactString > for & str { fn into_owned (self) -> compact_str :: CompactString { compact_str :: CompactString :: new (self) } }
    };
}

impl_211!()