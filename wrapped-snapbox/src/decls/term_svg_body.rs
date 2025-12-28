macro_rules! term_svg_body {
    () => {
        # [cfg (feature = "term-svg")] fn term_svg_body (svg : & str) -> Option < & str > { let (_header , body , _footer) = split_term_svg (svg) ? ; Some (body) }
    };
}

term_svg_body!()