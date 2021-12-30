#[non_exhaustive]
pub enum Signal{
    USART(Usart),
    CAN
}
pub struct Usart {
    pub receiver: bool,
    pub name: String,
    pub baudrate: u32,
}

pub struct Pin {
    pub name: String,
    pub alias: Option<String>,
    pub signal: Signal,
}

impl Pin{
    pub fn new(name: &str, alias: Option<&str>, signal: Signal) -> Self {
        Self {
            name: name.to_string(),
            alias:
                if let Some(a) = alias { Some(a.to_string()) }
                else { None },
            signal,
        }
    }
}