macro_rules! doc {
    () => {
        # [cfg (all (docsrs , unix))] pub mod doc ;
    };
}

doc!()