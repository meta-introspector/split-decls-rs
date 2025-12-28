macro_rules! deps {
    () => {
        FileId!();
        AnchoredPath!();
    };
}

macro_rules! AnchoredPathBuf {
    () => {
        deps!();
        # [doc = " Path relative to a file."] # [doc = ""] # [doc = " Owned version of [`AnchoredPath`]."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct AnchoredPathBuf { # [doc = " File that this path is relative to."] pub anchor : FileId , # [doc = " Path relative to `anchor`'s containing directory."] pub path : String , }
    };
}

AnchoredPathBuf!();