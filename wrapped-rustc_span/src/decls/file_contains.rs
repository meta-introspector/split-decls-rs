macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! file_contains {
    () => {
        deps!();
        # [inline] fn file_contains (file : & SourceFile , pos : BytePos) -> bool { file . contains (pos) && ! file . is_empty () }
    };
}

file_contains!()