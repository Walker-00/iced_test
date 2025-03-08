use iced::Alignment;
use iced::widget::{Column, button, column, container, row, text};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MainUi {
    pub value: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Page {
    Feed,
    Detail,
    Create,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    Inc,
    Dec,
}

impl MainUi {
    const FONT: &'static [u8] =
        include_bytes!("/usr/share/fonts/TTF/CaskaydiaCoveNerdFontMono-Regular.ttf");
    pub fn view(&self) -> Column<Message> {
        column![
            container(row![text("Simple Counter App").size(30)])
                .align_x(Alignment::Center)
                .width(iced::Length::Fill)
                .padding(10),
            button("+").on_press(Message::Inc),
            text(self.value).size(50),
            button("-").on_press(Message::Dec)
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Inc => {
                self.value += 1;
            }
            Message::Dec => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application("Test", MainUi::update, MainUi::view)
        .font(MainUi::FONT)
        .run()
}
