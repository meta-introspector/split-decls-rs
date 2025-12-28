macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! SourceFileAndLine {
    () => {
        deps!();
        # [derive (Debug)] pub struct SourceFileAndLine { pub sf : Arc < SourceFile > , # [doc = " Index of line, starting from 0."] pub line : usize , }
    };
}

SourceFileAndLine!();