macro_rules! deps {
    () => {
        LineInfo!();
        SourceFile!();
    };
}

macro_rules! FileLines {
    () => {
        deps!();
        pub struct FileLines { pub file : Arc < SourceFile > , pub lines : Vec < LineInfo > , }
    };
}

FileLines!()