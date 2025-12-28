macro_rules! deps {
    () => {
        Keyword!();
        Symbol!();
    };
}

macro_rules! Input {
    () => {
        deps!();
        struct Input { keywords : Punctuated < Keyword , Token ! [,] > , symbols : Punctuated < Symbol , Token ! [,] > , }
    };
}

Input!();