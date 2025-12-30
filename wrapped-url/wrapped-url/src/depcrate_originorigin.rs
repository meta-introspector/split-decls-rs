// Generated macro for Origin (enum)
macro_rules! Depcrate_originOrigin {
() => {
// Module: crate::origin
// Provides: {"Origin"}
// Dependencies: {}
# [doc = " The origin of an URL"] # [doc = ""] # [doc = " Two URLs with the same origin are considered"] # [doc = " to originate from the same entity and can therefore trust"] # [doc = " each other."] # [doc = ""] # [doc = " The origin is determined based on the scheme as follows:"] # [doc = ""] # [doc = " - If the scheme is \"blob\" the origin is the origin of the"] # [doc = "   URL contained in the path component. If parsing fails,"] # [doc = "   it is an opaque origin."] # [doc = " - If the scheme is \"ftp\", \"http\", \"https\", \"ws\", or \"wss\","] # [doc = "   then the origin is a tuple of the scheme, host, and port."] # [doc = " - If the scheme is anything else, the origin is opaque, meaning"] # [doc = "   the URL does not have the same origin as any other URL."] # [doc = ""] # [doc = " For more information see <https://url.spec.whatwg.org/#origin>"] # [derive (PartialEq , Eq , Hash , Clone , Debug)] pub enum Origin { # [doc = " A globally unique identifier"] Opaque (OpaqueOrigin) , # [doc = " Consists of the URL's scheme, host and port"] Tuple (String , Host < String > , u16) , }
};
}
