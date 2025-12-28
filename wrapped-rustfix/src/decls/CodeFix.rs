macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! CodeFix {
    () => {
        deps!();
        # [doc = " Represents a code fix. This doesn't write to disks but is only in memory."] # [doc = ""] # [doc = " The general way to use this is:"] # [doc = ""] # [doc = " 1. Feeds the source of a file to [`CodeFix::new`]."] # [doc = " 2. Calls [`CodeFix::apply`] to apply suggestions to the source code."] # [doc = " 3. Calls [`CodeFix::finish`] to get the \"fixed\" code."] # [derive (Clone)] pub struct CodeFix { data : replace :: Data , # [doc = " Whether or not the data has been modified."] modified : bool , }
    };
}

CodeFix!()