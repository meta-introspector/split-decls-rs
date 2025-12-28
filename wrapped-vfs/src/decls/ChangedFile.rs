macro_rules! deps {
    () => {
        Vfs!();
        Change!();
        FileId!();
    };
}

macro_rules! ChangedFile {
    () => {
        deps!();
        # [doc = " Changed file in the [`Vfs`]."] # [derive (Debug)] pub struct ChangedFile { # [doc = " Id of the changed file"] pub file_id : FileId , # [doc = " Kind of change"] pub change : Change , }
    };
}

ChangedFile!()