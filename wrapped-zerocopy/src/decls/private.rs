macro_rules! deps {
    () => {
        LittleEndian!();
        BigEndian!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait Sealed { } impl Sealed for super :: BigEndian { } impl Sealed for super :: LittleEndian { } }
    };
}

private!()