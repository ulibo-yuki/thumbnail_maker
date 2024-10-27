pub fn svg_maker(number: &String, subtitle: &String) -> svg::Document {
    let num_text = svg::node::Text::new(number);
    let title_text = svg::node::Text::new("Rustを分りたい");
    let sub_title_text = svg::node::Text::new(subtitle);

    let number_text = svg::node::element::Text::new("")
    .set("x", "640")
    .set("y", "125")
    .set("text-anchor", "middle")
    .set("fill", "#333333")
    .set("font-family", "Hiragino Sans")
    .set("font-weight", "Bold")
    .set("font-size", "64")
    .add(num_text);

    let title_text = svg::node::element::Text::new("")
        .set("x", "640")
        .set("y", "370")
        .set("text-anchor", "middle")
        .set("fill", "#333333")
        .set("font-family", "Hiragino Sans")
        .set("font-weight", "Bold")
        .set("font-size", 144)
        .add(title_text);

    let sub_title_text = svg::node::element::Text::new("")
        .set("x", "640")
        .set("y", "550")
        .set("text-anchor", "middle")
        .set("fill", "#333333")
        .set("font-family", "Hiragino Sans")
        .set("font-weight", "Bold")
        .set("font-size", 70)
        .add(sub_title_text);

    let background = svg::node::element::Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("fill", "#ffffff")
        .set("width", 1680)
        .set("height", 670);

    svg::Document::new()
        .set("viewBox", (0, 0, 1280, 670))
        .add(background)
        .add(number_text)
        .add(title_text)
        .add(sub_title_text)
}
