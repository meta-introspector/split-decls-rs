macro_rules! deps {
    () => {
        Directories!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A set of files on the file system."] # [derive (Debug , Clone)] pub enum Entry { # [doc = " The `Entry` is represented by a raw set of files."] Files (Vec < AbsPathBuf >) , # [doc = " The `Entry` is represented by `Directories`."] Directories (Directories) , }
    };
}

Entry!();