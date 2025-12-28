macro_rules! deps {
    () => {
        UInt!();
        Bit!();
        InvertedUnsigned!();
    };
}

macro_rules! InvertedUInt {
    () => {
        deps!();
        # [doc = " Inverted `UInt` (has most significant digit on the outside)"] pub struct InvertedUInt < IU : InvertedUnsigned , B : Bit > { msb : IU , lsb : B , }
    };
}

InvertedUInt!()