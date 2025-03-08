use iced::widget::{Column, Container, Image, button, column, container, row, text};
use iced::{Alignment, Theme};

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
    const FONT: &'static [u8] = include_bytes!("/usr/share/fonts/TTF/ZedMonoNerdFont-Regular.ttf");
    pub fn view(&self) -> Column<Message> {
        column![self.header(), self.body()]
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

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    pub fn header(&self) -> Container<Message> {
        container(row![text("Simple Counter App").size(30)])
            .align_x(Alignment::Center)
            .width(iced::Length::Fill)
            .padding(10)
    }

    pub fn body(&self) -> Column<Message> {
        column![
            Image::new("/home/walker/github/dotfiles/Wallpapers/buddha.jpg")
                .width(500)
                .height(300),
            button("+").on_press(Message::Inc),
            text(self.value).size(50),
            button("-").on_press(Message::Dec)
        ]
    }
}

fn main() -> iced::Result {
    iced::application("Test", MainUi::update, MainUi::view)
        .font(MainUi::FONT)
        .theme(MainUi::theme)
        .run()
}
