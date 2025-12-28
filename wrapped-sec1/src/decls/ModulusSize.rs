macro_rules! ModulusSize {
    () => {
        # [doc = " Trait for supported modulus sizes which precomputes the typenums for various point encodings so"] # [doc = " they don't need to be included as bounds."] pub trait ModulusSize : 'static + ArraySize + Copy + Debug + Add < U1 , Output = Self :: CompressedPointSize > { # [doc = " Size of a compressed point for the given elliptic curve when encoded using the SEC1"] # [doc = " `Elliptic-Curve-Point-to-Octet-String` algorithm (including leading `0x02` or `0x03`"] # [doc = " tag byte)."] type CompressedPointSize : 'static + ArraySize + Copy + Debug + Add < Self , Output = Self :: UncompressedPointSize > ; # [doc = " Size of an uncompressed point for the given elliptic curve when encoded using the SEC1"] # [doc = " `Elliptic-Curve-Point-to-Octet-String` algorithm (including leading `0x04` tag byte)."] type UncompressedPointSize : 'static + ArraySize + Copy + Debug ; # [doc = " Size of an untagged point for given elliptic curve, i.e. size of two serialized base field"] # [doc = " elements when concatenated."] type UntaggedPointSize : 'static + ArraySize + Copy + Debug + Sub < Self , Output = Self > ; }
    };
}

ModulusSize!()