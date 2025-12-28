macro_rules! register_usize {
    () => {
        # [doc = " Registers an action to set the flag to the given value whenever the signal arrives."] pub fn register_usize (signal : c_int , flag : Arc < AtomicUsize > , value : usize) -> Result < SigId , Error > { unsafe { low_level :: register (signal , move | | flag . store (value , Ordering :: SeqCst)) } }
    };
}

register_usize!()