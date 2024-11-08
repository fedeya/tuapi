use cached::proc_macro::cached;
use once_cell::sync::Lazy;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

pub static PS: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
pub static TS: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[cached]
pub fn highlight_response(response: String, content_type: String) -> Vec<Line<'static>> {
    let syntax_name = match content_type.as_str() {
        "application/json" => "json",
        "application/xml" => "xml",
        "text/html" => "html",
        "text/plain" => "txt",
        _ => "txt",
    };

    let syntax = PS.find_syntax_by_extension(syntax_name).unwrap();
    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);

    let mut lines: Vec<Line> = Vec::new();

    for line in LinesWithEndings::from(response.as_str()) {
        let ranges: Vec<(syntect::highlighting::Style, &str)> =
            h.highlight_line(line, &PS).unwrap();

        let spans: Vec<Span> = ranges
            .iter()
            .map(|segment| {
                let (style, content) = segment;

                Span::styled(
                    content.to_string(),
                    Style {
                        fg: translate_colour(style.foreground),
                        ..Style::default()
                    },
                )
            })
            .collect();

        lines.push(Line::from(spans));
    }

    lines
}

pub fn translate_colour(syntect_color: syntect::highlighting::Color) -> Option<Color> {
    match syntect_color {
        syntect::highlighting::Color { r, g, b, a } if a > 0 => Some(Color::Rgb(r, g, b)),
        _ => None,
    }
}
