use iced::widget::{Column, Container, Image, column, container, row, scrollable, text};
use iced::{Alignment, Element, Theme};

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
    pub fn view(&self) -> Element<Message> {
        let post_column = scrollable(column(
            (0..5).map(|_| container(self.body()).padding(10).into()),
        ));

        column![self.header(), post_column].into()
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
        let image = Image::new("/home/walker/github/dotfiles/Wallpapers/buddha.jpg")
            .width(300)
            .height(150)
            .content_fit(iced::ContentFit::Fill);

        let text_content = text("Hello World!").size(20);

        let post = container(column![image, text_content].spacing(10).padding(10))
            .width(320)
            .padding(5)
            .style(container::rounded_box);

        column![post]
    }
}

fn main() -> iced::Result {
    iced::application("Test", MainUi::update, MainUi::view)
        .font(MainUi::FONT)
        .theme(MainUi::theme)
        .run()
}
