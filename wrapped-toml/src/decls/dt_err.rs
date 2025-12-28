macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! dt_err {
    () => {
        deps!();
        fn dt_err (err : toml_datetime :: ser :: SerializerError) -> Error { match err { toml_datetime :: ser :: SerializerError :: InvalidFormat (err) => Error :: new (err) , _ => Error :: date_invalid () , } }
    };
}

dt_err!()