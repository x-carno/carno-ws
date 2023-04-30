use iced::widget::pane_grid::{self, Pane, PaneGrid};

// #[derive(Default)]
pub struct CarnoWS {
    pub panes: pane_grid::State<DisplayPane>,
    pub pane_focus: Option<pane_grid::Pane>,
    pub clients: Vec<Client>,
}

pub struct Client {
    pub client: String,
    pub checked: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum CarnoWSMessage {
    ClientChecked(bool),
    // SendPressed,
    AppLoad,
    PaneClicked(pane_grid::Pane),
}

pub struct DisplayPane {
    pub id: usize,
    pub is_pinned: bool,
}

impl DisplayPane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}
