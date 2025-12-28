macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! SourceFileAndBytePos {
    () => {
        deps!();
        # [derive (Debug)] pub struct SourceFileAndBytePos { pub sf : Arc < SourceFile > , pub pos : BytePos , }
    };
}

SourceFileAndBytePos!()