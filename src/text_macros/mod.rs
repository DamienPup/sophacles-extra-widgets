//! Macros for building and styling text for tui.

/// Styles text into a span with the bold modifier set. The argument must evaluate to something
/// that implements [`Into<Span>`](ratatui::text::Span)
#[macro_export]
macro_rules! bold {
    ($e:expr) => {{
        let mut s = ::ratatui::text::Span::from($e);
        s.style = s.style.add_modifier(::ratatui::style::Modifier::BOLD);
        s
    }};
}

/// Styles text into a span with the italic modifier set. The argument must evaluate to something
/// that implements [`Into<Span>`](ratatui::text::Span)
#[macro_export]
macro_rules! italic {
    ($e:expr) => {{
        let mut s = ::ratatui::text::Span::from($e);
        s.style = s.style.add_modifier(::ratatui::style::Modifier::ITALIC);
        s
    }};
}

/// Styles text into a span with the underlined modifier set. The argument must evaluate to something
/// that implements [`Into<Span>`](ratatui::text::Span)
#[macro_export]
macro_rules! underlined {
    ($e:expr) => {{
        let mut s = ::ratatui::text::Span::from($e);
        s.style = s.style.add_modifier(::ratatui::style::Modifier::UNDERLINED);
        s
    }};
}

/// Styles text into a span with the foreground set. The first argument must evaluate to something
/// that implements [`Into<Span>`](ratatui::text::Span), and the second a [`Color`](ratatui::style::Color)
#[macro_export]
macro_rules! fg {
    ($t:expr, $c: expr) => {{
        let mut s = ::ratatui::text::Span::from($t);
        s.style = s.style.fg($c);
        s
    }};
}

/// Styles text into a span with the background set. The first argument must evaluate to something
/// that implements [`Into<Span>`](ratatui::text::Span), and the second a [Color](ratatui::style::Color)
#[macro_export]
macro_rules! bg {
    ($t:expr, $c: expr) => {{
        let mut s = ::ratatui::text::Span::from($t);
        s.style = s.style.bg($c);
        s
    }};
}

/// Trait to allow all the overloading of the add_lines method
/// This is a helper to simplify the [text!](crate::text!) macro, and should not be used directly.
pub trait AddLines<T> {
    fn add_lines(&mut self, to_add: T);
}

impl<'a> AddLines<&'a str> for ::ratatui::text::Text<'a> {
    fn add_lines(&mut self, to_add: &'a str) {
        self.lines.push(to_add.into());
    }
}

impl<'a> AddLines<String> for ::ratatui::text::Text<'a> {
    fn add_lines(&mut self, to_add: String) {
        self.lines.push(to_add.into());
    }
}

impl<'a> AddLines<::ratatui::text::Span<'a>> for ::ratatui::text::Text<'a> {
    fn add_lines(&mut self, to_add: ::ratatui::text::Span<'a>) {
        self.lines.push(to_add.into());
    }
}

impl<'a> AddLines<::ratatui::text::Line<'a>> for ::ratatui::text::Text<'a> {
    fn add_lines(&mut self, to_add: ::ratatui::text::Line<'a>) {
        self.lines.push(to_add);
    }
}

impl<'a> AddLines<Vec<::ratatui::text::Line<'a>>> for ::ratatui::text::Text<'a> {
    fn add_lines(&mut self, mut to_add: Vec<::ratatui::text::Line<'a>>) {
        self.lines.append(&mut to_add);
    }
}

/// Create a [`Vec<Line>`](ratatui::text::Line) from lines of a string separated by '\n'
#[macro_export]
macro_rules! split {
    ($e:expr) => {{
        $e.lines()
            .map(|l| ::ratatui::text::Line::from(l))
            .collect::<Vec<::ratatui::text::Line>>()
    }};
}

/// Create a single [Line](ratatui::text::Line) from many
/// [Span](ratatui::text::Span) structs. Useful with [`text!`](crate::text!)
/// for having multiple stylings in a single line
#[macro_export]
macro_rules! line {
    ($($e:expr),* $(,)?) => {{
        let mut res = ::ratatui::text::Line::default();
        $(res.push_span(::ratatui::text::Span::from($e));)*;
        res
    }};
}

/// Creates a [`Vec<Line>`](ratatui::text::Line) from each line of the enclosed block
#[macro_export]
macro_rules! text {
    ($t:expr) => {
        res.push(Line::from($t));
    };
    ($($t:expr);* $(;)?) => {{
        use $crate::text_macros::AddLines;
        let mut res = ::ratatui::text::Text::default();
        $(res.add_lines($t);)*
        res
    }};
}

#[cfg(test)]
mod tests {
    use ratatui::{
        style::{Modifier, Style},
        text::{Span, Line, Text},
    };

    #[test]
    fn bold() {
        let expected = Span::styled("foo", Style::default().add_modifier(Modifier::BOLD));
        let test = bold!("foo");
        assert_eq!(expected, test);
    }

    #[test]
    fn italic() {
        let expected = Span::styled("foo", Style::default().add_modifier(Modifier::ITALIC));
        let test = italic!("foo");
        assert_eq!(expected, test);
    }

    #[test]
    fn underline() {
        let expected = Span::styled("foo", Style::default().add_modifier(Modifier::UNDERLINED));
        let test = underlined!("foo");
        assert_eq!(expected, test);
    }

    #[test]
    fn bold_italic() {
        let expected = Span::styled(
            "foo",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        );
        let test = bold!(italic!("foo"));
        assert_eq!(expected, test);
    }

    #[test]
    fn text() {
        let mut expected = Text::from(vec![
            Line::from(Span::styled(
                "foo",
                Style::default().add_modifier(Modifier::ITALIC),
            )),
            Line::from(Span::styled(
                "bar",
                Style::default().add_modifier(Modifier::UNDERLINED),
            )),
            Line::from("baz"),
        ]);

        let test = text! {
            italic!("foo");
            underlined!("bar");
            "baz";
        };
        assert_eq!(expected, test);

        let test = text! {
            italic!("foo");
            underlined!("bar");
            "baz"
        };
        assert_eq!(expected, test);

        let test = text! {
            italic!("foo");
            underlined!("bar");
            "baz";
            "a\nb";
            split!("q\nr")
        };
        expected.lines.push(Line::from("a\nb"));
        expected.lines.push(Line::from("q"));
        expected.lines.push(Line::from("r"));
        assert_eq!(expected, test);
    }

    #[test]
    fn text_single_line() {
        let expected = Text::from(vec![Line::from(Span::styled(
            "foo",
            Style::default().add_modifier(Modifier::ITALIC),
        ))]);

        let test = text! {
            italic!("foo");
        };

        assert_eq!(expected, test);
    }
}
