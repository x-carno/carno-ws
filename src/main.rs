use iced::widget::{checkbox, column, container};
use iced::{Element, Font, Length, Sandbox, Settings};

use app::{CarnoWS, CarnoWSMessage, Client};
mod app;

const ICON_FONT: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub fn main() -> iced::Result {
    CarnoWS::run(Settings::default())
}

impl Sandbox for CarnoWS {
    type Message = CarnoWSMessage;

    fn new() -> CarnoWS {
        let local_client = Client {
            client: String::from("127.0.0.1"),
            checked: false,
        };
        CarnoWS {
            clients: vec![local_client],
        }
    }

    fn title(&self) -> String {
        String::from("A websocket debug tool")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&self) -> Element<Self::Message> {
        // "Hello, world!".into()

        let client_checkbox = checkbox(
            self.clients[0].client.to_string(),
            self.clients[0].checked,
            CarnoWSMessage::ClientChecked,
        );
        // let cbx = vec![client_checkbox];

        // column(children)
        let content = column![client_checkbox]; //.spacing(22);
                                                // content.

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            // .center_x()
            .padding(10)
            // .center_y()
            .into()
    }
}
