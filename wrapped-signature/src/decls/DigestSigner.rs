macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! DigestSigner {
    () => {
        deps!();
        # [doc = " Sign the given prehashed message `Digest` using `Self`."] # [doc = ""] # [doc = " ## Notes"] # [doc = ""] # [doc = " This trait is primarily intended for signature algorithms based on the"] # [doc = " [Fiat-Shamir heuristic], a method for converting an interactive"] # [doc = " challenge/response-based proof-of-knowledge protocol into an offline"] # [doc = " digital signature through the use of a random oracle, i.e. a digest"] # [doc = " function."] # [doc = ""] # [doc = " The security of such protocols critically rests upon the inability of"] # [doc = " an attacker to solve for the output of the random oracle, as generally"] # [doc = " otherwise such signature algorithms are a system of linear equations and"] # [doc = " therefore doing so would allow the attacker to trivially forge signatures."] # [doc = ""] # [doc = " To prevent misuse which would potentially allow this to be possible, this"] # [doc = " API accepts a `Digest` instance, rather than a raw digest value."] # [doc = ""] # [doc = " [Fiat-Shamir heuristic]: https://en.wikipedia.org/wiki/Fiat%E2%80%93Shamir_heuristic"] # [cfg (feature = "digest")] pub trait DigestSigner < D : Update , S > { # [doc = " Sign a message by updating the received `Digest` with it,"] # [doc = " returning a signature."] # [doc = ""] # [doc = " The given function can be invoked multiple times. It is expected that"] # [doc = " in each invocation the `Digest` is updated with the entire equal message."] # [doc = ""] # [doc = " Panics in the event of a signing error."] fn sign_digest < F : Fn (& mut D) > (& self , f : F) -> S { self . try_sign_digest (| digest | { f (digest) ; Ok (()) }) . expect ("signature operation failed") } # [doc = " Attempt to sign a message by updating the received `Digest` with it,"] # [doc = " returning a digital signature on success, or an error if something went wrong."] # [doc = ""] # [doc = " The given function can be invoked multiple times. It is expected that"] # [doc = " in each invocation the `Digest` is updated with the entire equal message."] fn try_sign_digest < F : Fn (& mut D) -> Result < () , Error > > (& self , f : F) -> Result < S , Error > ; }
    };
}

DigestSigner!();