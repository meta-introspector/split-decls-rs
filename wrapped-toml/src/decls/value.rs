macro_rules! value {
    () => {
        # [cfg (feature = "serde")] pub mod value ;
    };
}

value!()