macro_rules! thread_id {
    () => {
        # [doc = " The thread identifier of the calling thread."] pub fn thread_id () -> u32 { unsafe { GetCurrentThreadId () } }
    };
}

thread_id!();