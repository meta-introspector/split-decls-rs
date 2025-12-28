macro_rules! ChangeKind {
    () => {
        # [doc = " Kind of [file change](ChangedFile)."] # [derive (Eq , PartialEq , Debug)] pub enum ChangeKind { # [doc = " The file was (re-)created"] Create , # [doc = " The file was modified"] Modify , # [doc = " The file was deleted"] Delete , }
    };
}

ChangeKind!();