macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! SignerMut {
    () => {
        deps!();
        # [doc = " Sign the provided message bytestring using `&mut Self` (e.g. an evolving"] # [doc = " cryptographic key such as a stateful hash-based signature), returning a"] # [doc = " digital signature."] pub trait SignerMut < S > { # [doc = " Sign the given message, update the state, and return a digital signature."] fn sign (& mut self , msg : & [u8]) -> S { self . try_sign (msg) . expect ("signature operation failed") } # [doc = " Attempt to sign the given message, updating the state, and returning a"] # [doc = " digital signature on success, or an error if something went wrong."] # [doc = ""] # [doc = " Signing can fail, e.g., if the number of time periods allowed by the"] # [doc = " current key is exceeded."] fn try_sign (& mut self , msg : & [u8]) -> Result < S , Error > ; }
    };
}

SignerMut!();