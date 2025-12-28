macro_rules! deps {
    () => {
        StreamStatus!();
        Stdio!();
    };
}

macro_rules! Stream {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] struct Stream { stream : Stdio , content : crate :: Data , status : StreamStatus , }
    };
}

Stream!();