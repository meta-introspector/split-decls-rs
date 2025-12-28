macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! ZipResult {
    () => {
        deps!();
        # [doc = " Generic result type with ZipError as its error variant"] pub type ZipResult < T > = Result < T , ZipError > ;
    };
}

ZipResult!();