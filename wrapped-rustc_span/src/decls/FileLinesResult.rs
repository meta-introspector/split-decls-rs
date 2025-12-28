macro_rules! deps {
    () => {
        FileLines!();
        SpanLinesError!();
    };
}

macro_rules! FileLinesResult {
    () => {
        deps!();
        pub type FileLinesResult = Result < FileLines , SpanLinesError > ;
    };
}

FileLinesResult!()