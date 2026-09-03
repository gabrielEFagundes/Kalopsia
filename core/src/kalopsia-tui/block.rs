use ratatui::{Frame, layout::Rect, style::Style, widgets::{Block, BorderType}};

pub struct KBlock{
    area: Rect,
    style: Style,
    title: String,
}

impl KBlock{
    pub fn new(area: Rect, style: Style) -> Self{
        Self { area, style, title: "".to_string() }
    }

    pub fn set_title(&mut self, title: &str) -> &Self{
        self.title = title.to_string();
        self
    }

    pub fn render(self, frame: &mut Frame){
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(self.title.clone())
                .style(self.style),
                self.area
        );
    }
}