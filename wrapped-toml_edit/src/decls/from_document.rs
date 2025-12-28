macro_rules! deps {
    () => {
        Error!();
        Deserializer!();
    };
}

macro_rules! from_document {
    () => {
        deps!();
        # [doc = " Convert a [`DocumentMut`][crate::DocumentMut] into `T`."] pub fn from_document < T > (d : impl Into < Deserializer >) -> Result < T , Error > where T : DeserializeOwned , { let deserializer = d . into () ; T :: deserialize (deserializer) }
    };
}

from_document!()