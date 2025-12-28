macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl Fields { fn new (cs : & 'static dyn Callsite) -> Self { let fieldset = cs . metadata () . fields () ; let message = fieldset . field ("message") . unwrap () ; let target = fieldset . field ("log.target") . unwrap () ; let module = fieldset . field ("log.module_path") . unwrap () ; let file = fieldset . field ("log.file") . unwrap () ; let line = fieldset . field ("log.line") . unwrap () ; Fields { message , target , module , file , line , } } }
    };
}

impl_38!();