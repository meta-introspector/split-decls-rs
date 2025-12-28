macro_rules! deps {
    () => {
        ByteOrder!();
    };
}

macro_rules! LittleEndian {
    () => {
        deps!();
        # [doc = " Little-endian byte order."] # [doc = ""] # [doc = " See [`ByteOrder`] for more details."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum LittleEndian { }
    };
}

LittleEndian!();