macro_rules! deps {
    () => {
        ExecuteSequencesError!();
        DecompressBlockError!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl From < ExecuteSequencesError > for DecompressBlockError { fn from (val : ExecuteSequencesError) -> Self { Self :: ExecuteSequencesError (val) } }
    };
}

impl_56!()