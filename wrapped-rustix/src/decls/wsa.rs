macro_rules! wsa {
    () => {
        # [cfg (windows)] mod wsa ;
    };
}

wsa!()