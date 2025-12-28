macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! InvalidUuid {
    () => {
        deps!();
        # [doc = " A string that is guaranteed to fail to parse to a [`Uuid`]."] # [doc = ""] # [doc = " This type acts as a lightweight error indicator, suggesting"] # [doc = " that the string cannot be parsed but offering no error"] # [doc = " details. To get details, use `InvalidUuid::into_err`."] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct InvalidUuid < 'a > (pub (crate) & 'a [u8]) ;
    };
}

InvalidUuid!()