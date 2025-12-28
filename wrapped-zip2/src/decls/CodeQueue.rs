macro_rules! CodeQueue {
    () => {
        struct CodeQueue { next_idx : usize , codes : [Option < u16 > ; FREE_CODE_QUEUE_SIZE] , }
    };
}

CodeQueue!()