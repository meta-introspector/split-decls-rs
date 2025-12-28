macro_rules! deps {
    () => {
        Error!();
        SequencesHeaderParseError!();
        ExecuteSequencesError!();
        DecompressLiteralsError!();
        LiteralsSectionParseError!();
        DecodeSequenceError!();
    };
}

macro_rules! DecompressBlockError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum DecompressBlockError { BlockContentReadError (Error) , MalformedSectionHeader { expected_len : usize , remaining_bytes : usize , } , DecompressLiteralsError (DecompressLiteralsError) , LiteralsSectionParseError (LiteralsSectionParseError) , SequencesHeaderParseError (SequencesHeaderParseError) , DecodeSequenceError (DecodeSequenceError) , ExecuteSequencesError (ExecuteSequencesError) , }
    };
}

DecompressBlockError!();