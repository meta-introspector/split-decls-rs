// Generated macro for html (function)
macro_rules! Depcrate_replyhtml {
() => {
// Module: crate::reply
// Provides: {"html"}
// Dependencies: {}
# [doc = " Reply with a body and `content-type` set to `text/html; charset=utf-8`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let body = r#\""] # [doc = " <html>"] # [doc = "     <head>"] # [doc = "         <title>HTML with warp!</title>"] # [doc = "     </head>"] # [doc = "     <body>"] # [doc = "         <h1>warp + HTML = &hearts;</h1>"] # [doc = "     </body>"] # [doc = " </html>"] # [doc = " \"#;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(move || {"] # [doc = "         warp::reply::html(body)"] # [doc = "     });"] # [doc = " ```"] pub fn html < T > (body : T) -> Html < T > where crate :: bodyt :: Body : From < T > , T : Send , { Html { body } }
};
}
