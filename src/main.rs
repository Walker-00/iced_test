use iced::widget::{Column, Container, Image, Row, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length, Theme};

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
        let posts: Vec<u8> = (0..30).collect();
        let post_per_row: Vec<Row<Message>> =
            posts.chunks(3).map(|v| row![self.body(&v[0])]).collect();
        let mut tp = 0;
        let post_column = scrollable(
            column((0..(posts.len() - tp)).map(|i| {
                row((0..3).map(|_| {
                    let body = self.body(&posts[tp]).into();
                    if tp < posts.len() - 1 {
                        tp += 1;
                    }
                    body
                }))
                .into()
            })), // row((0..3).map(|_| {
                 //     column(posts.iter().map(|i| self.body(i).into()))
                 //         .width(Length::Fill)
                 //         .into()
                 // }))
                 // .spacing(10),
        )
        .width(Length::Fill)
        .height(Length::Fill);

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

    pub fn body(&self, texts: &u8) -> Column<Message> {
        let image = Image::new("/home/walker/github/dotfiles/Wallpapers/buddha.jpg")
            .width(300)
            .height(150)
            .content_fit(iced::ContentFit::Fill);

        let text_content = text(texts).size(20);

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
