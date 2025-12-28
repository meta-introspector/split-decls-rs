macro_rules! deps {
    () => {
        FileName!();
    };
}

macro_rules! MalformedSourceMapPositions {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] pub struct MalformedSourceMapPositions { pub name : FileName , pub source_len : usize , pub begin_pos : BytePos , pub end_pos : BytePos , }
    };
}

MalformedSourceMapPositions!();