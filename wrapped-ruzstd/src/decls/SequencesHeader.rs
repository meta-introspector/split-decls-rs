macro_rules! deps {
    () => {
        CompressionModes!();
    };
}

macro_rules! SequencesHeader {
    () => {
        deps!();
        pub struct SequencesHeader { pub num_sequences : u32 , pub modes : Option < CompressionModes > , }
    };
}

SequencesHeader!()