macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! NetworkEndian {
    () => {
        deps!();
        # [doc = " The endianness used in many network protocols."] # [doc = ""] # [doc = " This is a type alias for [`BigEndian`]."] pub type NetworkEndian = BigEndian ;
    };
}

NetworkEndian!();