macro_rules! Protocol {
    () => {
        # [doc = " Protocol specification used for creating sockets via `Socket::new`."] # [doc = ""] # [doc = " This is a newtype wrapper around an integer which provides a nicer API in"] # [doc = " addition to an injection point for documentation."] # [doc = ""] # [doc = " This type is freely interconvertible with C's `int` type, however, if a raw"] # [doc = " value needs to be provided."] # [derive (Copy , Clone , Eq , PartialEq)] pub struct Protocol (c_int) ;
    };
}

Protocol!()