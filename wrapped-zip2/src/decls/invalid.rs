macro_rules! invalid {
    () => {
        macro_rules ! invalid { ($ message : literal) => { crate :: result :: invalid_archive_const ($ message) } ; ($ ($ arg : tt) *) => { crate :: result :: invalid_archive (format ! ($ ($ arg) *)) } ; }
    };
}

invalid!()