macro_rules! deps {
    () => {
        DiagnosticDeriveVariantBuilder!();
        DiagnosticDeriveKind!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl DiagnosticDeriveKind { # [doc = " Call `f` for the struct or for each variant of the enum, returning a `TokenStream` with the"] # [doc = " tokens from `f` wrapped in an `match` expression. Emits errors for use of derive on unions"] # [doc = " or attributes on the type itself when input is an enum."] pub (crate) fn each_variant < 's , F > (self , structure : & mut Structure < 's > , f : F) -> TokenStream where F : for < 'v > Fn (DiagnosticDeriveVariantBuilder , & VariantInfo < 'v >) -> TokenStream , { let ast = structure . ast () ; let span = ast . span () . unwrap () ; match ast . data { syn :: Data :: Struct (..) | syn :: Data :: Enum (..) => () , syn :: Data :: Union (..) => { span_err (span , "diagnostic derives can only be used on structs and enums") . emit () ; } } if matches ! (ast . data , syn :: Data :: Enum (..)) { for attr in & ast . attrs { span_err (attr . span () . unwrap () , "unsupported type attribute for diagnostic derive enum" ,) . emit () ; } } structure . bind_with (| _ | synstructure :: BindStyle :: Move) ; let variants = structure . each_variant (| variant | { let span = match structure . ast () . data { syn :: Data :: Struct (..) => span , _ => variant . ast () . ident . span () . unwrap () , } ; let builder = DiagnosticDeriveVariantBuilder { kind : self , span , field_map : build_field_mapping (variant) , formatting_init : TokenStream :: new () , slug : None , code : None , } ; f (builder , variant) }) ; quote ! { match self { # variants } } } }
    };
}

impl_15!()