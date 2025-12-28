macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! Signer {
    () => {
        deps!();
        # [doc = " Sign the provided message bytestring using `Self` (e.g. a cryptographic key"] # [doc = " or connection to an HSM), returning a digital signature."] pub trait Signer < S > { # [doc = " Sign the given message and return a digital signature"] fn sign (& self , msg : & [u8]) -> S { self . try_sign (msg) . expect ("signature operation failed") } # [doc = " Attempt to sign the given message, returning a digital signature on"] # [doc = " success, or an error if something went wrong."] # [doc = ""] # [doc = " The main intended use case for signing errors is when communicating"] # [doc = " with external signers, e.g. cloud KMS, HSMs, or other hardware tokens."] fn try_sign (& self , msg : & [u8]) -> Result < S , Error > ; }
    };
}

Signer!();