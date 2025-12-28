macro_rules! deps {
    () => {
        BufferData!();
    };
}

macro_rules! Buffer {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq)] struct Buffer { offset : usize , data : BufferData , }
    };
}

Buffer!();