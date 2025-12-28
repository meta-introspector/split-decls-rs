macro_rules! deps {
    () => {
        FSETable!();
        FSEScratch!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl FSEScratch { pub fn new () -> FSEScratch { FSEScratch { offsets : FSETable :: new (MAX_OFFSET_CODE) , of_rle : None , literal_lengths : FSETable :: new (MAX_LITERAL_LENGTH_CODE) , ll_rle : None , match_lengths : FSETable :: new (MAX_MATCH_LENGTH_CODE) , ml_rle : None , } } pub fn reinit_from (& mut self , other : & Self) { self . offsets . reinit_from (& other . offsets) ; self . literal_lengths . reinit_from (& other . literal_lengths) ; self . match_lengths . reinit_from (& other . match_lengths) ; self . of_rle = other . of_rle ; self . ll_rle = other . ll_rle ; self . ml_rle = other . ml_rle ; } }
    };
}

impl_175!()