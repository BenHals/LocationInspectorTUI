pub enum Message {
    Quit,
    Select,
    Back,
    Tab,
    ShiftUp,
    ShiftDown,
    ShiftLeft,
    ShiftRight,
    ZoomIn,
    ZoomOut,
    ListUp,
    ListDown,
    /// Synthetic — dispatched to a screen when it becomes active.
    Activated,
}
