macro_rules! Thread {
    () => {
        # [doc = " Data which is unique to the current thread while it is running."] # [doc = " A thread ID may be reused after a thread exits."] # [derive (Clone , Copy)] pub (crate) struct Thread { # [doc = " The bucket this thread's local storage will be in."] pub (crate) bucket : usize , # [doc = " The index into the bucket this thread's local storage is in."] pub (crate) index : usize , }
    };
}

Thread!()