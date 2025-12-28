macro_rules! deps {
    () => {
        SourceFile!();
        SpanDecoder!();
        NormalizedPos!();
        MultiByteChar!();
        SourceFileDiffs!();
        SourceFileLines!();
        SourceFileHash!();
        ExternalSource!();
        FileName!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < D : SpanDecoder > Decodable < D > for SourceFile { fn decode (d : & mut D) -> SourceFile { let name : FileName = Decodable :: decode (d) ; let src_hash : SourceFileHash = Decodable :: decode (d) ; let checksum_hash : Option < SourceFileHash > = Decodable :: decode (d) ; let source_len : RelativeBytePos = Decodable :: decode (d) ; let lines = { let num_lines : u32 = Decodable :: decode (d) ; if num_lines > 0 { let bytes_per_diff = d . read_u8 () as usize ; let num_diffs = num_lines as usize - 1 ; let raw_diffs = d . read_raw_bytes (bytes_per_diff * num_diffs) . to_vec () ; SourceFileLines :: Diffs (SourceFileDiffs { bytes_per_diff , num_diffs , raw_diffs }) } else { SourceFileLines :: Lines (vec ! []) } } ; let multibyte_chars : Vec < MultiByteChar > = Decodable :: decode (d) ; let stable_id = Decodable :: decode (d) ; let normalized_pos : Vec < NormalizedPos > = Decodable :: decode (d) ; let cnum : CrateNum = Decodable :: decode (d) ; SourceFile { name , start_pos : BytePos :: from_u32 (0) , source_len , src : None , src_hash , checksum_hash , external_src : FreezeLock :: frozen (ExternalSource :: Unneeded) , lines : FreezeLock :: new (lines) , multibyte_chars , normalized_pos , stable_id , cnum , } } }
    };
}

impl_309!()