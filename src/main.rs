use iced::widget::{Column, Container, Image, MouseArea, column, container, row, scrollable, text};
use iced::{Alignment, Element, Font, Length, Settings, Theme, window};

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
    Pressed(u8),
}

impl MainUi {
    const FONT: &'static [u8] = include_bytes!("/usr/share/fonts/TTF/ZedMonoNerdFont-Regular.ttf");

    pub fn view(&self) -> Element<Message> {
        let posts: Vec<u8> = (0..31).collect();
        let post_per_row = posts.chunks(3);
        let post_column = scrollable(column(post_per_row.map(|v| {
            row(v.iter().map(|i| {
                MouseArea::new(self.body(i).padding(5))
                    .on_press(Message::Pressed(*i))
                    .into()
            }))
            .into()
        })))
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
            Message::Pressed(u) => {
                // println!("{u}");
            }
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    pub fn header(&self) -> Element<Message> {
        row![
            container(text("👁️").font(Font::with_name(name)).size(20))
                .align_y(Alignment::Center)
                .padding(10),
            container(text("Simple Counter App").size(30))
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .width(iced::Length::Fill)
                .padding(10)
        ]
        .into()
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
