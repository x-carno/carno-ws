#[derive(Default)]
pub struct CarnoWS {
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
}
