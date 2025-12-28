macro_rules! ZipCryptoKeys {
    () => {
        # [doc = " A container to hold the current key state"] # [cfg_attr (fuzzing , derive (arbitrary :: Arbitrary))] # [derive (Clone , Copy , Hash , Ord , PartialOrd , Eq , PartialEq)] pub (crate) struct ZipCryptoKeys { key_0 : Wrapping < u32 > , key_1 : Wrapping < u32 > , key_2 : Wrapping < u32 > , }
    };
}

ZipCryptoKeys!();