macro_rules! deps {
    () => {
        ByteOrder!();
    };
}

macro_rules! BigEndian {
    () => {
        deps!();
        # [doc = " Big-endian byte order."] # [doc = ""] # [doc = " See [`ByteOrder`] for more details."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum BigEndian { }
    };
}

BigEndian!()