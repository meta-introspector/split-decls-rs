macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! ErasedFileAstId {
    () => {
        deps!();
        # [doc = " This is a type erased FileAstId."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct ErasedFileAstId (u32) ;
    };
}

ErasedFileAstId!()