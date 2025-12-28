macro_rules! deps {
    () => {
        FrameDescriptor!();
        Read!();
        FrameHeader!();
        ReadFrameHeaderError!();
    };
}

macro_rules! read_frame_header {
    () => {
        deps!();
        # [doc = " Read a single serialized frame from the reader and return a tuple containing the parsed frame and the number of bytes read."] pub fn read_frame_header (mut r : impl Read) -> Result < (FrameHeader , u8) , ReadFrameHeaderError > { use ReadFrameHeaderError as err ; let mut buf = [0u8 ; 4] ; r . read_exact (& mut buf) . map_err (err :: MagicNumberReadError) ? ; let mut bytes_read = 4 ; let magic_num = u32 :: from_le_bytes (buf) ; if (0x184D2A50 ..= 0x184D2A5F) . contains (& magic_num) { r . read_exact (& mut buf) . map_err (err :: FrameDescriptorReadError) ? ; let skip_size = u32 :: from_le_bytes (buf) ; return Err (ReadFrameHeaderError :: SkipFrame { magic_number : magic_num , length : skip_size , }) ; } if magic_num != MAGIC_NUM { return Err (ReadFrameHeaderError :: BadMagicNumber (magic_num)) ; } r . read_exact (& mut buf [0 .. 1]) . map_err (err :: FrameDescriptorReadError) ? ; let desc = FrameDescriptor (buf [0]) ; bytes_read += 1 ; let mut frame_header = FrameHeader { descriptor : FrameDescriptor (desc . 0) , dict_id : None , frame_content_size : 0 , window_descriptor : 0 , } ; if ! desc . single_segment_flag () { r . read_exact (& mut buf [0 .. 1]) . map_err (err :: WindowDescriptorReadError) ? ; frame_header . window_descriptor = buf [0] ; bytes_read += 1 ; } let dict_id_len = desc . dictionary_id_bytes () ? as usize ; if dict_id_len != 0 { let buf = & mut buf [.. dict_id_len] ; r . read_exact (buf) . map_err (err :: DictionaryIdReadError) ? ; bytes_read += dict_id_len ; let mut dict_id = 0u32 ; # [allow (clippy :: needless_range_loop)] for i in 0 .. dict_id_len { dict_id += (buf [i] as u32) << (8 * i) ; } if dict_id != 0 { frame_header . dict_id = Some (dict_id) ; } } let fcs_len = desc . frame_content_size_bytes () ? as usize ; if fcs_len != 0 { let mut fcs_buf = [0u8 ; 8] ; let fcs_buf = & mut fcs_buf [.. fcs_len] ; r . read_exact (fcs_buf) . map_err (err :: FrameContentSizeReadError) ? ; bytes_read += fcs_len ; let mut fcs = 0u64 ; # [allow (clippy :: needless_range_loop)] for i in 0 .. fcs_len { fcs += (fcs_buf [i] as u64) << (8 * i) ; } if fcs_len == 2 { fcs += 256 ; } frame_header . frame_content_size = fcs ; } Ok ((frame_header , bytes_read as u8)) }
    };
}

read_frame_header!();