macro_rules! deps {
    () => {
        SourceFileDiffs!();
    };
}

macro_rules! SourceFileLines {
    () => {
        deps!();
        # [derive (Clone)] pub enum SourceFileLines { # [doc = " The source file lines, in decoded (random-access) form."] Lines (Vec < RelativeBytePos >) , # [doc = " The source file lines, in undecoded difference list form."] Diffs (SourceFileDiffs) , }
    };
}

SourceFileLines!();