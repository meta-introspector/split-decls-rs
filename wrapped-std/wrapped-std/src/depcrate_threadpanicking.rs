// Generated macro for panicking (function)
macro_rules! Depcrate_threadpanicking {
() => {
// Module: crate::thread
// Provides: {"panicking"}
// Dependencies: {}
# [doc = " Determines whether the current thread is unwinding because of panic."] # [doc = ""] # [doc = " A common use of this feature is to poison shared resources when writing"] # [doc = " unsafe code, by checking `panicking` when the `drop` is called."] # [doc = ""] # [doc = " This is usually not needed when writing safe code, as [`Mutex`es][Mutex]"] # [doc = " already poison themselves when a thread panics while holding the lock."] # [doc = ""] # [doc = " This can also be used in multithreaded applications, in order to send a"] # [doc = " message to other threads warning that a thread has panicked (e.g., for"] # [doc = " monitoring purposes)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " struct SomeStruct;"] # [doc = ""] # [doc = " impl Drop for SomeStruct {"] # [doc = "     fn drop(&mut self) {"] # [doc = "         if thread::panicking() {"] # [doc = "             println!(\"dropped while unwinding\");"] # [doc = "         } else {"] # [doc = "             println!(\"dropped while not unwinding\");"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " {"] # [doc = "     print!(\"a: \");"] # [doc = "     let a = SomeStruct;"] # [doc = " }"] # [doc = ""] # [doc = " {"] # [doc = "     print!(\"b: \");"] # [doc = "     let b = SomeStruct;"] # [doc = "     panic!()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [Mutex]: crate::sync::Mutex"] # [inline] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn panicking () -> bool { panicking :: panicking () }
};
}
