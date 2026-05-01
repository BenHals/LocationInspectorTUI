pub enum Message {
    // primitive key events
    Char(char),
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Tab,

    // top-level
    Quit,

    // synthetic — dispatched to a screen when it becomes active
    Activated,
}
