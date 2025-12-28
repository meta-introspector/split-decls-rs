macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! MultiByteChar {
    () => {
        deps!();
        # [doc = " Identifies an offset of a multi-byte character in a `SourceFile`."] # [derive (Copy , Clone , Encodable , Decodable , Eq , PartialEq , Debug , HashStable_Generic)] pub struct MultiByteChar { # [doc = " The relative offset of the character in the `SourceFile`."] pub pos : RelativeBytePos , # [doc = " The number of bytes, `>= 2`."] pub bytes : u8 , }
    };
}

MultiByteChar!();