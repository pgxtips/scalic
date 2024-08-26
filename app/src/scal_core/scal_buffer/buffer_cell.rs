#[derive(Debug)]
pub struct BufferCell {
    pub character: char,
    pub fg_color: (u8, u8, u8, u8),
    pub bg_color: (u8, u8, u8, u8),
    pub is_cursor_blink: bool,
    pub is_wireframe: bool,
}
