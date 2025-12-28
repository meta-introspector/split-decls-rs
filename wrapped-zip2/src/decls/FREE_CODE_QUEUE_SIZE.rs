macro_rules! FREE_CODE_QUEUE_SIZE {
    () => {
        # [doc = " Number of codes available for the LZW dictionary (excluding control codes)"] # [doc = " These are the codes from CONTROL_CODE+1 to MAX_CODE"] const FREE_CODE_QUEUE_SIZE : usize = MAX_CODE - CONTROL_CODE + 1 ;
    };
}

FREE_CODE_QUEUE_SIZE!()