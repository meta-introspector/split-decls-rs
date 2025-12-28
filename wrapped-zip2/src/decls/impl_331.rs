macro_rules! deps {
    () => {
        CodeQueue!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl CodeQueue { fn new () -> Self { let mut codes = [None ; FREE_CODE_QUEUE_SIZE] ; for (i , code) in ((CONTROL_CODE as u16 + 1) ..= (MAX_CODE as u16 - 1)) . enumerate () { codes [i] = Some (code) ; } Self { next_idx : 0 , codes } } fn next (& self) -> Option < u16 > { if let Some (Some (next)) = self . codes . get (self . next_idx) { Some (* next) } else { None } } # [doc = " Return and remove the next code from the queue, or return INVALID_CODE if"] # [doc = " the queue is empty."] fn remove_next (& mut self) -> Option < u16 > { let res = self . next () ; if res . is_some () { self . next_idx += 1 ; } res } }
    };
}

impl_331!();