macro_rules! deps {
    () => {
        FileName!();
    };
}

macro_rules! DistinctSources {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] pub struct DistinctSources { pub begin : (FileName , BytePos) , pub end : (FileName , BytePos) , }
    };
}

DistinctSources!();