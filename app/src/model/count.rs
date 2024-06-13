

#[derive(Debug, Clone, PartialEq)]
pub enum CountAction {
    Increase,
    Decrease,
    Reset
}

#[derive(Debug, Clone, PartialEq)]
pub struct CountEvent {
    pub id: String,
    pub action: CountAction
}