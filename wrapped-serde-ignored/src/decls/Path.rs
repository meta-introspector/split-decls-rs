macro_rules! Path {
    () => {
        # [doc = " Path to the current value in the input, like `dependencies.serde.typo1`."] pub enum Path < 'a > { Root , Seq { parent : & 'a Path < 'a > , index : usize } , Map { parent : & 'a Path < 'a > , key : String } , Some { parent : & 'a Path < 'a > } , NewtypeStruct { parent : & 'a Path < 'a > } , NewtypeVariant { parent : & 'a Path < 'a > } , }
    };
}

Path!()