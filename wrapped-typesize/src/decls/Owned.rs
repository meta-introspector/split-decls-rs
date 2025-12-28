macro_rules! deps {
    () => {
        SizableArc!();
        SizableRc!();
    };
}

macro_rules! Owned {
    () => {
        deps!();
        # [doc = " Marker type for reference counted types such as [`SizableRc`] or [`SizableArc`]"] pub struct Owned ;
    };
}

Owned!();