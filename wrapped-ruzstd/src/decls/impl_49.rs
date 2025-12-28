macro_rules! deps {
    () => {
        DecompressLiteralsError!();
        Error!();
        LiteralsSectionParseError!();
        ExecuteSequencesError!();
        DecodeSequenceError!();
        DecompressBlockError!();
        SequencesHeaderParseError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for DecompressBlockError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecompressBlockError :: BlockContentReadError (source) => Some (source) , DecompressBlockError :: DecompressLiteralsError (source) => Some (source) , DecompressBlockError :: LiteralsSectionParseError (source) => Some (source) , DecompressBlockError :: SequencesHeaderParseError (source) => Some (source) , DecompressBlockError :: DecodeSequenceError (source) => Some (source) , DecompressBlockError :: ExecuteSequencesError (source) => Some (source) , _ => None , } } }
    };
}

impl_49!()