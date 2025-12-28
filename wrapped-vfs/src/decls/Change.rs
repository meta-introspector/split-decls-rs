macro_rules! Change {
    () => {
        # [doc = " Kind of [file change](ChangedFile)."] # [derive (Eq , PartialEq , Debug)] pub enum Change { # [doc = " The file was (re-)created"] Create (Vec < u8 > , u64) , # [doc = " The file was modified"] Modify (Vec < u8 > , u64) , # [doc = " The file was deleted"] Delete , }
    };
}

Change!();