// Generated macro for visit_content_map (function)
macro_rules! Depcrate_contentvisit_content_map {
() => {
// Module: crate::content
// Provides: {"visit_content_map"}
// Dependencies: {}
fn visit_content_map < 'de , V , E > (content : Vec < (Content < 'de > , Content < 'de >) > , visitor : V ,) -> Result < V :: Value , E > where V : Visitor < 'de > , E : de :: Error , { let map = content . into_iter () . map (| (k , v) | (ContentDeserializer :: new (k) , ContentDeserializer :: new (v))) ; let mut map_visitor = value :: MapDeserializer :: new (map) ; let value = visitor . visit_map (& mut map_visitor) ? ; map_visitor . end () ? ; Ok (value) }
};
}
