macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! Verifier {
    () => {
        deps!();
        # [doc = " Verify the provided message bytestring using `Self` (e.g. a public key)"] pub trait Verifier < S > { # [doc = " Use `Self` to verify that the provided signature for a given message"] # [doc = " bytestring is authentic."] # [doc = ""] # [doc = " Returns `Error` if it is inauthentic, or otherwise returns `()`."] fn verify (& self , msg : & [u8] , signature : & S) -> Result < () , Error > ; }
    };
}

Verifier!();