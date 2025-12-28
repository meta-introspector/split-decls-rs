macro_rules! tokio_thread_local {
    () => {
        # [cfg (not (all (loom , test)))] macro_rules ! tokio_thread_local { ($ ($ tts : tt) +) => { :: std :: thread_local ! { $ ($ tts) + } } }
    };
}

tokio_thread_local!();