// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
use chrono::Local;
use iced::alignment::{self, Alignment};
use iced::theme::{self, Text, Theme};
use iced::widget::pane_grid::{self, Axis, Pane, PaneGrid};
use iced::widget::{button, checkbox, column, container, scrollable, text, TextInput};
use iced::widget::{Checkbox, Column, Row, Scrollable};
use iced::{Application, Command, Element, Font, Length, Sandbox, Settings};
use iced_lazy::responsive;
use log::debug;
use std::time::SystemTime;

use app::{CarnoWS, CarnoWSMessage, Client, DisplayPane};
mod app;

const ICON_FONT: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub fn main() -> iced::Result {
    CarnoWS::run(Settings::default())
}

impl Application for CarnoWS {
    type Message = CarnoWSMessage;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<CarnoWSMessage>) {
        println!("init app");
        let local_client = Client {
            client: String::from("127.0.0.1"),
            checked: false,
        };

        let (mut panes, first) = pane_grid::State::new(DisplayPane::new(0));
        // let left_pane = Pane(1);
        panes.split(pane_grid::Axis::Vertical, &first, DisplayPane::new(1));

        panes.split(pane_grid::Axis::Vertical, &first, DisplayPane::new(2));

        (
            CarnoWS {
                panes: panes,
                pane_focus: None,
                clients: vec![local_client],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A websocket debug tool")
    }

    fn update(&mut self, _message: CarnoWSMessage) -> Command<CarnoWSMessage> {
        // This application has no interactions

        match _message {
            CarnoWSMessage::ClientChecked(value) => {
                self.clients[0].checked = value;
            } //::(value) => self.default_checkbox = value,
            // Message::CustomChecked(value) => self.custom_checkbox = value,
            CarnoWSMessage::AppLoad => {}
            CarnoWSMessage::PaneClicked(pane) => {
                self.pane_focus = Some(pane);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let focus = self.pane_focus;
        // "Hello, world!".into()
        // println!("generate view");

        // let left_grid = pane_grid::Content::new(responsive(move |size|{
        //     clients_view_content()
        // }))
        // .title_bar(pane_grid::TitleBar::new(text("clients")));

        // let content = Row::new().push(Column::new()).push(Column::new());

        // PaneGrid::new(, view)
        let ws_pane = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let is_focus = focus == Some(id);

            pane_grid::Content::new(responsive(move |size| view_contents(size, pane.id))).style(
                if is_focus {
                    style::pane_focused
                } else {
                    style::pane_active
                },
            )
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_click(CarnoWSMessage::PaneClicked)
        .spacing(10);

        container(ws_pane)
            .width(Length::Fill)
            .height(Length::Fill)
            // .center_x()
            .padding(10)
            // .center_y()
            .into()
    }
}

fn view_contents<'a>(size: iced::Size, display_pane_id: usize) -> Element<'a, CarnoWSMessage> {
    if display_pane_id == 0 {
        let local_check = Checkbox::new("localhost", false, CarnoWSMessage::ClientChecked);

        let content = Column::new().push(local_check);

        container(scrollable(content)).into()
    } else if display_pane_id == 2 {
        let send_message = TextInput::new("message to send", "value");

        let content = Column::new().push(send_message);
        container(scrollable(content)).into()
    } else {
        let local_check = Checkbox::new("localhost", false, CarnoWSMessage::ClientChecked);

        let content = Column::new().push(local_check);
        container(scrollable(content)).into()
    }
}

mod style {
    use iced::widget::container;
    use iced::Theme;

    pub fn title_bar_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border_width: 2.0,
            border_color: palette.background.strong.color,
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border_width: 2.0,
            border_color: palette.primary.strong.color,
            ..Default::default()
        }
    }
}
