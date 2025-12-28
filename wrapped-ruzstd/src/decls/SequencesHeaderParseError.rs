macro_rules! SequencesHeaderParseError {
    () => {
        # [derive (Debug)] # [non_exhaustive] pub enum SequencesHeaderParseError { NotEnoughBytes { need_at_least : u8 , got : usize } , }
    };
}

SequencesHeaderParseError!();