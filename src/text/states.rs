
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TextRenderingState {
    Pendding(String),
    Rendering,
    Done,
}
