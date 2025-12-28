macro_rules! deps {
    () => {
        ExtendedFileOptions!();
        FileOptions!();
    };
}

macro_rules! FullFileOptions {
    () => {
        deps!();
        # [doc = " Adds Extra Data and Central Extra Data. It does not implement copy."] pub type FullFileOptions < 'k > = FileOptions < 'k , ExtendedFileOptions > ;
    };
}

FullFileOptions!();