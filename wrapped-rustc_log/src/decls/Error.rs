macro_rules! Error {
    () => {
        # [derive (Debug)] pub enum Error { InvalidColorValue (String) , NonUnicodeColorValue , InvalidWraptree (String) , AlreadyInit (SetGlobalDefaultError) , }
    };
}

Error!()