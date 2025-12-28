macro_rules! Domain {
    () => {
        # [doc = " Specification of the communication domain for a socket."] # [doc = ""] # [doc = " This is a newtype wrapper around an integer which provides a nicer API in"] # [doc = " addition to an injection point for documentation. Convenience constants such"] # [doc = " as [`Domain::IPV4`], [`Domain::IPV6`], etc, are provided to avoid reaching"] # [doc = " into libc for various constants."] # [doc = ""] # [doc = " This type is freely interconvertible with C's `int` type, however, if a raw"] # [doc = " value needs to be provided."] # [derive (Copy , Clone , Eq , PartialEq)] pub struct Domain (c_int) ;
    };
}

Domain!();