macro_rules! RowId {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq)] struct RowId { val : isize , }
    };
}

RowId!();