macro_rules! tinyvec {
    () => {
        # [cfg (feature = "alloc")] mod tinyvec ;
    };
}

tinyvec!()