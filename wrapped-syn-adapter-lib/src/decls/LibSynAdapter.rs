macro_rules! LibSynAdapter {
    () => {
        # [cfg (feature = "syn-parsing")] pub struct LibSynAdapter ;
    };
}

LibSynAdapter!()