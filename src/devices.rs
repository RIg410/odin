use transport::Transport;

#[derive(Debug)]
pub struct SerialSwitch {
    id: String,
    p_id: u8,
    transport: Transport,
}

impl SerialSwitch {
    pub fn new(transport: &Transport, id: &str, p_id: u8) -> SerialSwitch {
        SerialSwitch {
            id: id.to_owned(),
            transport: transport.clone(),
            p_id,
        }
    }
}

#[derive(Debug)]
pub struct SerialDimmer {
    id: String,
    p_id: u8,
    transport: Transport,
    min_value: u8,
    max_value: u8,
}

impl SerialDimmer {
    pub fn new(transport: &Transport, id: &str, p_id: u8, min_value: u8, max_value: u8) -> SerialDimmer {
        SerialDimmer {
            id: id.to_owned(),
            transport: transport.clone(),
            p_id,
            min_value,
            max_value,
        }
    }
}

#[derive(Debug)]
pub struct WebBeam {
    id: String,
    transport: Transport,
}

impl WebBeam {
    pub fn new(transport: &Transport, id: &str) -> WebBeam {
        WebBeam {
            transport: transport.clone(),
            id: id.to_owned(),
        }
    }
}