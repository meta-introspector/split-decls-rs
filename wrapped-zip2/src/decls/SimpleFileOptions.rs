macro_rules! deps {
    () => {
        FileOptions!();
    };
}

macro_rules! SimpleFileOptions {
    () => {
        deps!();
        # [doc = " Simple File Options. Can be copied and good for simple writing zip files"] pub type SimpleFileOptions = FileOptions < 'static , () > ;
    };
}

SimpleFileOptions!()