macro_rules! BorrowedFieldProjectionMode {
    () => {
        # [doc = " When checking for borrows of field projections (`&(*ptr).a`), we might want"] # [doc = " to check for the field type (type of `.a` in the example). This enum defines"] # [doc = " the variations (pass the pointer [Ty] or the field [Ty])."] # [derive (Copy , Clone)] pub (crate) enum BorrowedFieldProjectionMode { FollowProjections , NoFollowProjections , }
    };
}

BorrowedFieldProjectionMode!()