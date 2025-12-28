macro_rules! deps {
    () => {
        Palette!();
        Styled!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl Palette { pub fn color () -> Self { if cfg ! (feature = "color") { Self { info : anstyle :: AnsiColor :: Green . on_default () , warn : anstyle :: AnsiColor :: Yellow . on_default () , error : anstyle :: AnsiColor :: Red . on_default () , hint : anstyle :: Effects :: DIMMED . into () , expected : anstyle :: AnsiColor :: Red . on_default () | anstyle :: Effects :: UNDERLINE , actual : anstyle :: AnsiColor :: Green . on_default () | anstyle :: Effects :: UNDERLINE , } } else { Self :: plain () } } pub fn plain () -> Self { Self :: default () } pub fn info < D : std :: fmt :: Display > (self , item : D) -> Styled < D > { Styled :: new (item , self . info) } pub fn warn < D : std :: fmt :: Display > (self , item : D) -> Styled < D > { Styled :: new (item , self . warn) } pub fn error < D : std :: fmt :: Display > (self , item : D) -> Styled < D > { Styled :: new (item , self . error) } pub fn hint < D : std :: fmt :: Display > (self , item : D) -> Styled < D > { Styled :: new (item , self . hint) } pub fn expected < D : std :: fmt :: Display > (self , item : D) -> Styled < D > { Styled :: new (item , self . expected) } pub fn actual < D : std :: fmt :: Display > (self , item : D) -> Styled < D > { Styled :: new (item , self . actual) } }
    };
}

impl_330!();