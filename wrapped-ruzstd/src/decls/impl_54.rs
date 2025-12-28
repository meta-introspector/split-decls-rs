macro_rules! deps {
    () => {
        DecompressBlockError!();
        SequencesHeaderParseError!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl From < SequencesHeaderParseError > for DecompressBlockError { fn from (val : SequencesHeaderParseError) -> Self { Self :: SequencesHeaderParseError (val) } }
    };
}

impl_54!();