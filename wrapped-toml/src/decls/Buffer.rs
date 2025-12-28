macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! Buffer {
    () => {
        deps!();
        # [doc = " TOML Document serialization buffer"] # [derive (Debug , Default)] pub struct Buffer { tables : Vec < Option < Table > > , }
    };
}

Buffer!();