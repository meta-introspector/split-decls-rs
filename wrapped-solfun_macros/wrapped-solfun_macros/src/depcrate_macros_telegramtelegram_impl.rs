// Generated macro for telegram_impl (function)
macro_rules! Depcrate_macros_telegramtelegram_impl {
() => {
// Module: crate::macros::telegram
// Provides: {"telegram_impl"}
// Dependencies: {}
# [decl (fn , name = "telegram_impl" , vis = "pub" , hash = "90c2aa78")] pub fn telegram_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n✉️ TELEGRAM! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
};
}
