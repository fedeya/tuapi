use cached::proc_macro::cached;
use once_cell::sync::Lazy;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};

static PS: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static TS: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[cached]
pub fn highlight_response(response: String) -> Vec<Line<'static>> {
    let syntax = PS.find_syntax_by_extension("json").unwrap();
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

fn translate_colour(syntect_color: syntect::highlighting::Color) -> Option<Color> {
    match syntect_color {
        syntect::highlighting::Color { r, g, b, a } if a > 0 => Some(Color::Rgb(r, g, b)),
        _ => None,
    }
}
