// Generated macro for get_response (function)
macro_rules! Depcrateget_response {
() => {
// Module: crate
// Provides: {"get_response"}
// Dependencies: {}
async fn get_response (location : & str) -> JsonValue { let url1 = "http://api.openweathermap.org/data/2.5/weather?q=" ; let url2 = "&appid=<apiKey>" ; let url = [url1 , location , url2] . concat () ; let resp = reqwest :: get (& url) . await . unwrap () . text () . await . unwrap () ; json :: parse (& resp) . unwrap () }
};
}
