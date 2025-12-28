macro_rules! deps {
    () => {
        ErasedFileAstId!();
        EditionedFileId!();
    };
}

macro_rules! RealSpanMap {
    () => {
        deps!();
        # [derive (PartialEq , Eq , Hash , Debug)] pub struct RealSpanMap { file_id : EditionedFileId , # [doc = " Invariant: Sorted vec over TextSize"] pairs : Box < [(TextSize , ErasedFileAstId)] > , end : TextSize , }
    };
}

RealSpanMap!();