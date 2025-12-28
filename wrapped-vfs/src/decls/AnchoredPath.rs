macro_rules! deps {
    () => {
        FileId!();
        AnchoredPathBuf!();
    };
}

macro_rules! AnchoredPath {
    () => {
        deps!();
        # [doc = " Path relative to a file."] # [doc = ""] # [doc = " Borrowed version of [`AnchoredPathBuf`]."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct AnchoredPath < 'a > { # [doc = " File that this path is relative to."] pub anchor : FileId , # [doc = " Path relative to `anchor`'s containing directory."] pub path : & 'a str , }
    };
}

AnchoredPath!();