macro_rules! OneshotWithSecretError {
    () => {
        # [doc = " The provided secret was not at least [`SECRET_MINIMUM_LENGTH`][]"] # [doc = " bytes."] # [derive (Debug)] pub struct OneshotWithSecretError (pub (crate) secret :: Error) ;
    };
}

OneshotWithSecretError!();