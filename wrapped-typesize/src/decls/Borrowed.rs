macro_rules! deps {
    () => {
        SizableArc!();
        SizableRc!();
    };
}

macro_rules! Borrowed {
    () => {
        deps!();
        # [doc = " Marker type for reference counted types such as [`SizableRc`] or [`SizableArc`]"] pub struct Borrowed ;
    };
}

Borrowed!()