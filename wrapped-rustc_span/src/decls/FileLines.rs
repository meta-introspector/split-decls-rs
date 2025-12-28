macro_rules! deps {
    () => {
        SourceFile!();
        LineInfo!();
    };
}

macro_rules! FileLines {
    () => {
        deps!();
        pub struct FileLines { pub file : Arc < SourceFile > , pub lines : Vec < LineInfo > , }
    };
}

FileLines!();