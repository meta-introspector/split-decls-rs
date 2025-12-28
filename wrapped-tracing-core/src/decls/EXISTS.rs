macro_rules! EXISTS {
    () => {
        static EXISTS : AtomicBool = AtomicBool :: new (false) ;
    };
}

EXISTS!()