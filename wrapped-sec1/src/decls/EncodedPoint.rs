macro_rules! deps {
    () => {
        ModulusSize!();
    };
}

macro_rules! EncodedPoint {
    () => {
        deps!();
        # [doc = " SEC1 encoded curve point."] # [doc = ""] # [doc = " This type is an enum over the compressed and uncompressed encodings,"] # [doc = " useful for cases where either encoding can be supported, or conversions"] # [doc = " between the two forms."] # [derive (Clone , Default)] pub struct EncodedPoint < Size > where Size : ModulusSize , { bytes : Array < u8 , Size :: UncompressedPointSize > , }
    };
}

EncodedPoint!()