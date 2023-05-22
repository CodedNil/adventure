use crate::data;
use iced::widget::{Column, Container, Row, Text, image};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub struct PlayerCharacterComponent {
    player: data::PlayerCharacter,
    is_expanded: bool,
}

enum Message {}

impl PlayerCharacterComponent {
    pub fn new(player: data::PlayerCharacter) -> Self {
        Self {
            player,
            is_expanded: false,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let image = image::new("assets/character.png").width(Length::from(100));

        let name = Text::new(self.player.name.clone());

        let mut content = Row::new().spacing(10).push(image).push(name);

        if self.is_expanded {
            let description = Text::new(self.player.description.clone());
            let items = Text::new(format!("Items: {:?}", self.player.items));

            content = content
                .push(Column::new().spacing(10).push(description).push(items))
                .width(Length::Fill);
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::from(100))
            .center_x()
            .center_y()
            .into()
    }

    pub fn update(&mut self) {
        self.is_expanded = !self.is_expanded;
    }
}
