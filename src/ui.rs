use iced::{
    widget::{Column, Container, Text},
    window, Application, Command, Element, Length, Settings, Subscription, Theme,
};
use iced_aw::{split, Split};
use iced_aw::{TabLabel, Tabs};
use std::cmp;

use crate::{Item, PlayerCharacter};

pub fn create(players: Vec<PlayerCharacter>) -> iced::Result {
    UIApp::run(Settings {
        flags: players,
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    OnJournalResize(u16),
    WindowResized(u32, u32),
    JournalTabSelected(usize),
}

struct UIApp {
    window_width: u16,
    main_width: u16,
    journal_width: u16,
    journal_tab: usize,
    players: Vec<PlayerCharacter>,
}

impl Application for UIApp {
    type Message = Message;
    type Flags = Vec<PlayerCharacter>;
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(players: Self::Flags) -> (UIApp, Command<Message>) {
        (
            UIApp {
                window_width: 800,
                main_width: 600,
                journal_width: 200,
                journal_tab: 0,
                players: players,
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
                let max_position = self.window_width - 200;
                let valid_position = cmp::min(position, max_position);
                self.journal_width = self.window_width - valid_position;
                self.main_width = cmp::max(self.window_width - self.journal_width, 200);
            }
            Message::WindowResized(width, _height) => {
                self.window_width = cmp::max(width as u16, 200);
                self.main_width =
                    cmp::max(self.window_width.saturating_sub(self.journal_width), 200);
            }
            Message::JournalTabSelected(index) => self.journal_tab = index,
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

        let tab_labels = vec!["Players", "Zones", "Factions", "Animals"];

        let mut journal = Tabs::new(self.journal_tab, Message::JournalTabSelected)
            .tab_bar_position(iced_aw::TabBarPosition::Top);

        for label in tab_labels {
            match label {
                "Players" => {
                    let tab_label = TabLabel::Text(label.to_string());
                    let mut column = Column::new();

                    for player in &self.players {
                        let player_text = format!(
                            "Name: {}\nDescription: {}\nStatus: {}\nItems: {:?}\n",
                            player.name, player.description, player.status, player.items
                        );
                        column = column.push(Text::new(player_text));
                    }
                    journal = journal.push(tab_label, column);
                }
                _ => {
                    let tab_label = TabLabel::Text(label.to_string());
                    let content = Container::new(Text::new(format!("{label} Content")));
                    journal = journal.push(tab_label, content);
                }
            }
        }

        Split::new(
            main_content,
            journal,
            Some(self.main_width),
            split::Axis::Vertical,
            Message::OnJournalResize,
        )
        .into()
    }
}
