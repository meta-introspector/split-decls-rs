macro_rules! ExtraFieldVersion {
    () => {
        # [doc = " marker trait to denote the place where this extra field has been stored"] pub trait ExtraFieldVersion { }
    };
}

ExtraFieldVersion!()