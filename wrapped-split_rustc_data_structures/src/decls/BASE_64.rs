macro_rules! BASE_64 {
    () => {
        const BASE_64 : [ascii :: Char ; MAX_BASE] = { let bytes = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@$" ; let Some (ascii) = bytes . as_ascii () else { panic ! () } ; * ascii } ;
    };
}

BASE_64!()