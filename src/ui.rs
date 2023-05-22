use iced::{
    widget::{Container, Text},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use iced_aw::{split, Split};
use std::cmp;

pub fn create() -> iced::Result {
    UIApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    OnJournalResize(u16),
    WindowResized(u32, u32),
}

struct UIApp {
    main_width: u16,
    journal_width: u16,
    window_width: u16,
}

impl Application for UIApp {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: Self::Flags) -> (UIApp, Command<Message>) {
        (
            UIApp {
                main_width: 600,
                journal_width: 200,
                window_width: 800,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Adventure")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnJournalResize(position) => {
                let max_position = self.window_width - 50;
                let valid_position = cmp::min(position, max_position);
                self.journal_width = self.window_width - valid_position;
                self.main_width = cmp::max(self.window_width - self.journal_width, 200);
            }
            Message::WindowResized(width, _height) => {
                self.window_width = cmp::max(width as u16, 100);
                self.main_width =
                    cmp::max(self.window_width.saturating_sub(self.journal_width), 200);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            iced_native::Event::Window(window::Event::Resized { width, height }) => {
                Some(Message::WindowResized(width, height))
            }
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let main_content = Container::new(Text::new("Main Content"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let journal = Container::new(Text::new("Journal"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        Split::new(
            main_content,
            journal,
            Some(self.main_width),
            split::Axis::Vertical,
            Message::OnJournalResize,
        )
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
