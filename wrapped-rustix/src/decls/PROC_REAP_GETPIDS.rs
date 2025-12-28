macro_rules! PROC_REAP_GETPIDS {
    () => {
        # [cfg (feature = "alloc")] const PROC_REAP_GETPIDS : c_int = 5 ;
    };
}

PROC_REAP_GETPIDS!();