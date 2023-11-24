pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(&self.label).unwrap();
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

const PANEL_PADDING: usize = 2;

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + PANEL_PADDING * 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut msg = String::with_capacity(self.label.width());
        self.label.draw_into(&mut msg);

        let outer_width = self.width();
        writeln!(buffer, "+{:-^outer_width$}+", "").unwrap();
        writeln!(buffer, "|{:^outer_width$}|", msg).unwrap();
        writeln!(buffer, "+{:-^outer_width$}+", "").unwrap();
    }
}

#[test]
fn test_widget() {
    let mut buff = String::new();
    let btn = Button::new("test");
    btn.draw_into(&mut buff);
    assert_eq!(btn.width(), 8);
    println!("{buff}");
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width() + PANEL_PADDING
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let inner_width = self.inner_width();
        let outer_width = inner_width + PANEL_PADDING * 2;
        let content_width = inner_width + PANEL_PADDING;

        // header
        writeln!(buffer, "+{:-^outer_width$}+", "").unwrap();
        writeln!(buffer, "|{:^outer_width$}|", &self.title).unwrap();
        writeln!(buffer, "+{:=^outer_width$}+", "").unwrap();

        let mut buff = String::with_capacity(inner_width);
        for widget in &self.widgets {
            buff.clear();
            widget.draw_into(&mut buff);

            // Draw & decorate every line
            buff.lines()
                .for_each(|line| writeln!(buffer, "| {:<content_width$} |", line).unwrap());
        }

        // Footer
        writeln!(buffer, "+{:-^outer_width$}+\n", "").unwrap();
    }
}

#[test]
fn test_main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
