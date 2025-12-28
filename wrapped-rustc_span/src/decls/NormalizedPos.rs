macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! NormalizedPos {
    () => {
        deps!();
        # [doc = " Identifies an offset of a character that was normalized away from `SourceFile`."] # [derive (Copy , Clone , Encodable , Decodable , Eq , PartialEq , Debug , HashStable_Generic)] pub struct NormalizedPos { # [doc = " The relative offset of the character in the `SourceFile`."] pub pos : RelativeBytePos , # [doc = " The difference between original and normalized string at position."] pub diff : u32 , }
    };
}

NormalizedPos!()