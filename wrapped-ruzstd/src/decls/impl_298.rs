macro_rules! deps {
    () => {
        SequencesHeaderParseError!();
        CompressionModes!();
        SequencesHeader!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl SequencesHeader { # [doc = " Create a new [SequencesHeader]."] pub fn new () -> SequencesHeader { SequencesHeader { num_sequences : 0 , modes : None , } } # [doc = " Attempt to deserialize the provided buffer into `self`, returning the number of bytes read."] pub fn parse_from_header (& mut self , source : & [u8]) -> Result < u8 , SequencesHeaderParseError > { let mut bytes_read = 0 ; if source . is_empty () { return Err (SequencesHeaderParseError :: NotEnoughBytes { need_at_least : 1 , got : 0 , }) ; } match source [0] { 0 => { self . num_sequences = 0 ; bytes_read += 1 ; } 1 ..= 127 => { if source . len () < 2 { return Err (SequencesHeaderParseError :: NotEnoughBytes { need_at_least : 2 , got : source . len () , }) ; } self . num_sequences = u32 :: from (source [0]) ; self . modes = Some (CompressionModes (source [1])) ; bytes_read += 2 ; } 128 ..= 254 => { if source . len () < 2 { return Err (SequencesHeaderParseError :: NotEnoughBytes { need_at_least : 2 , got : source . len () , }) ; } self . num_sequences = ((u32 :: from (source [0]) - 128) << 8) + u32 :: from (source [1]) ; bytes_read += 2 ; if self . num_sequences != 0 { if source . len () < 3 { return Err (SequencesHeaderParseError :: NotEnoughBytes { need_at_least : 3 , got : source . len () , }) ; } self . modes = Some (CompressionModes (source [2])) ; bytes_read += 1 ; } } 255 => { if source . len () < 4 { return Err (SequencesHeaderParseError :: NotEnoughBytes { need_at_least : 4 , got : source . len () , }) ; } self . num_sequences = u32 :: from (source [1]) + (u32 :: from (source [2]) << 8) + 0x7F00 ; self . modes = Some (CompressionModes (source [3])) ; bytes_read += 4 ; } } Ok (bytes_read) } }
    };
}

impl_298!();