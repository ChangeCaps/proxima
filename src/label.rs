#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Label(pub String);

impl Label {
    pub fn new(label: impl Into<String>) -> Self {
        Self(label.into())
    }
}
