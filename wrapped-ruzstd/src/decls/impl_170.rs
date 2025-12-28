macro_rules! deps {
    () => {
        FSEScratch!();
        FSETable!();
        HuffmanScratch!();
        DecodeBuffer!();
        DecoderScratch!();
        HuffmanTable!();
        Dictionary!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl DecoderScratch { pub fn new (window_size : usize) -> DecoderScratch { DecoderScratch { huf : HuffmanScratch { table : HuffmanTable :: new () , } , fse : FSEScratch { offsets : FSETable :: new (MAX_OFFSET_CODE) , of_rle : None , literal_lengths : FSETable :: new (MAX_LITERAL_LENGTH_CODE) , ll_rle : None , match_lengths : FSETable :: new (MAX_MATCH_LENGTH_CODE) , ml_rle : None , } , buffer : DecodeBuffer :: new (window_size) , offset_hist : [1 , 4 , 8] , block_content_buffer : Vec :: new () , literals_buffer : Vec :: new () , sequences : Vec :: new () , } } pub fn reset (& mut self , window_size : usize) { self . offset_hist = [1 , 4 , 8] ; self . literals_buffer . clear () ; self . sequences . clear () ; self . block_content_buffer . clear () ; self . buffer . reset (window_size) ; self . fse . literal_lengths . reset () ; self . fse . match_lengths . reset () ; self . fse . offsets . reset () ; self . fse . ll_rle = None ; self . fse . ml_rle = None ; self . fse . of_rle = None ; self . huf . table . reset () ; } pub fn init_from_dict (& mut self , dict : & Dictionary) { self . fse . reinit_from (& dict . fse) ; self . huf . table . reinit_from (& dict . huf . table) ; self . offset_hist = dict . offset_hist ; self . buffer . dict_content . clear () ; self . buffer . dict_content . extend_from_slice (& dict . dict_content) ; } }
    };
}

impl_170!()