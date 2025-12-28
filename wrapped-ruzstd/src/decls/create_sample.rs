macro_rules! deps {
    () => {
        Reservoir!();
        Read!();
    };
}

macro_rules! create_sample {
    () => {
        deps!();
        # [doc = " Creates a representative sample of `input` of `size` bytes."] pub fn create_sample < R : io :: Read > (input : & mut R , size : usize) -> Vec < u8 > { let reservoir = Reservoir :: new (size) ; reservoir . fill (input) }
    };
}

create_sample!();