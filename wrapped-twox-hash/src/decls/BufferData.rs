macro_rules! deps {
    () => {
        Lanes!();
    };
}

macro_rules! BufferData {
    () => {
        deps!();
        # [derive (Clone , PartialEq)] struct BufferData (Lanes) ;
    };
}

BufferData!();