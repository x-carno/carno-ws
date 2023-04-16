use iced::alignment;
use iced::executor;
use iced::widget::{button, checkbox, container, text, Column, Row};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use iced_native::layout::Node;
use iced_native::{Event, Layout};

pub fn main() -> iced::Result {
    Events::run(Settings {
        exit_on_close_request: false,
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Events {
    last: Vec<iced_native::Event>,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    Toggled(bool),
    Exit,
}

impl Application for Events {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (Events::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) if self.enabled => {
                self.last.push(event);

                if self.last.len() > 5 {
                    let _ = self.last.remove(0);
                }

                Command::none()
            }
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    Command::none()
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;

                Command::none()
            }
            Message::Exit => window::close(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let events = Column::with_children(
            self.last
                .iter()
                .map(|event| text(format!("{event:?}")).size(40))
                .map(Element::from)
                .collect(),
        );

        let toggle = checkbox("Listen to runtime events", self.enabled, Message::Toggled);

        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .width(100)
        .padding(10)
        .on_press(Message::Exit);

        // Layout::new(Node::new(10));

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(events)
            .push(toggle)
            .push(exit);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// use iced::widget::{checkbox, column, container, Column};
// use iced::{Element, Font, Length, Sandbox, Settings};

// use app::{CarnoWS, CarnoWSMessage, Client};
// mod app;

// const ICON_FONT: Font = Font::External {
//     name: "Icons",
//     bytes: include_bytes!("../fonts/icons.ttf"),
// };

// pub fn main() -> iced::Result {
//     CarnoWS::run(Settings::default())
// }

// impl Sandbox for CarnoWS {
//     type Message = CarnoWSMessage;

//     fn new() -> CarnoWS {
//         let local_client = Client {
//             client: String::from("127.0.0.1"),
//             checked: false,
//         };
//         CarnoWS {
//             clients: vec![local_client],
//         }
//     }

//     fn title(&self) -> String {
//         String::from("A websocket debug tool")
//     }

//     fn update(&mut self, _message: Self::Message) {
//         // This application has no interactions
//     }

//     fn view(&self) -> Element<Self::Message> {
//         // "Hello, world!".into()

//         let client_checkbox = checkbox(
//             self.clients[0].client.to_string(),
//             self.clients[0].checked,
//             CarnoWSMessage::ClientChecked,
//         );
//         // let cbx = vec![client_checkbox];

//         // column(children)
//         let content = column![client_checkbox]; //.spacing(22);
//                                                 // content.

//         container(content)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             // .center_x()
//             .padding(10)
//             // .center_y()
//             .into()
//     }
// }
