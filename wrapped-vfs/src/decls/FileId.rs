macro_rules! deps {
    () => {
        Vfs!();
    };
}

macro_rules! FileId {
    () => {
        deps!();
        # [doc = " Handle to a file in [`Vfs`]"] # [doc = ""] # [doc = " Most functions in rust-analyzer use this when they need to refer to a file."] # [derive (Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct FileId (u32) ;
    };
}

FileId!()