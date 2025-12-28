macro_rules! deps {
    () => {
        Sequence!();
        FseTableMode!();
        CompressState!();
        Matcher!();
        BitWriter!();
    };
}

macro_rules! compress_block {
    () => {
        deps!();
        # [doc = " A block of [`crate::common::BlockType::Compressed`]"] pub fn compress_block < M : Matcher > (state : & mut CompressState < M > , output : & mut Vec < u8 >) { let mut literals_vec = Vec :: new () ; let mut sequences = Vec :: new () ; state . matcher . start_matching (| seq | { match seq { Sequence :: Literals { literals } => literals_vec . extend_from_slice (literals) , Sequence :: Triple { literals , offset , match_len , } => { literals_vec . extend_from_slice (literals) ; sequences . push (crate :: blocks :: sequence_section :: Sequence { ll : literals . len () as u32 , ml : match_len as u32 , of : (offset + 3) as u32 , }) ; } } }) ; let mut writer = BitWriter :: from (output) ; if literals_vec . len () > 1024 { if let Some (table) = compress_literals (& literals_vec , state . last_huff_table . as_ref () , & mut writer) { state . last_huff_table . replace (table) ; } } else { raw_literals (& literals_vec , & mut writer) ; } if sequences . is_empty () { writer . write_bits (0u8 , 8) ; } else { encode_seqnum (sequences . len () , & mut writer) ; let ll_mode = choose_table (state . fse_tables . ll_previous . as_ref () , & state . fse_tables . ll_default , sequences . iter () . map (| seq | encode_literal_length (seq . ll) . 0) , 9 ,) ; let ml_mode = choose_table (state . fse_tables . ml_previous . as_ref () , & state . fse_tables . ml_default , sequences . iter () . map (| seq | encode_match_len (seq . ml) . 0) , 9 ,) ; let of_mode = choose_table (state . fse_tables . of_previous . as_ref () , & state . fse_tables . of_default , sequences . iter () . map (| seq | encode_offset (seq . of) . 0) , 8 ,) ; writer . write_bits (encode_fse_table_modes (& ll_mode , & ml_mode , & of_mode) , 8) ; encode_table (& ll_mode , & mut writer) ; encode_table (& of_mode , & mut writer) ; encode_table (& ml_mode , & mut writer) ; encode_sequences (& sequences , & mut writer , ll_mode . as_ref () , ml_mode . as_ref () , of_mode . as_ref () ,) ; if let FseTableMode :: Encoded (table) = ll_mode { state . fse_tables . ll_previous = Some (table) } if let FseTableMode :: Encoded (table) = ml_mode { state . fse_tables . ml_previous = Some (table) } if let FseTableMode :: Encoded (table) = of_mode { state . fse_tables . of_previous = Some (table) } } writer . flush () ; }
    };
}

compress_block!();