macro_rules! ReplaceReceiver {
    () => {
        struct ReplaceReceiver < 'a > (& 'a TypePath) ;
    };
}

ReplaceReceiver!();