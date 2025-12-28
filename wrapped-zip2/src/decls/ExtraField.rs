macro_rules! deps {
    () => {
        ExtendedTimestamp!();
        Ntfs!();
    };
}

macro_rules! ExtraField {
    () => {
        deps!();
        # [doc = " contains one extra field"] # [derive (Debug , Clone)] pub enum ExtraField { # [doc = " NTFS extra field"] Ntfs (Ntfs) , # [doc = " extended timestamp, as described in <https://libzip.org/specifications/extrafld.txt>"] ExtendedTimestamp (ExtendedTimestamp) , }
    };
}

ExtraField!()