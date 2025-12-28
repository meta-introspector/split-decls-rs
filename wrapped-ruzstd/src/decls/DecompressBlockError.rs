macro_rules! deps {
    () => {
        DecompressLiteralsError!();
        Error!();
        LiteralsSectionParseError!();
        ExecuteSequencesError!();
        DecodeSequenceError!();
        SequencesHeaderParseError!();
    };
}

macro_rules! DecompressBlockError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum DecompressBlockError { BlockContentReadError (Error) , MalformedSectionHeader { expected_len : usize , remaining_bytes : usize , } , DecompressLiteralsError (DecompressLiteralsError) , LiteralsSectionParseError (LiteralsSectionParseError) , SequencesHeaderParseError (SequencesHeaderParseError) , DecodeSequenceError (DecodeSequenceError) , ExecuteSequencesError (ExecuteSequencesError) , }
    };
}

DecompressBlockError!()