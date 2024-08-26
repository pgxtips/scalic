#[derive(Debug)]
pub struct BufferCell {
    pub character: char,
    pub fg_color: (u8, u8, u8, u8),
    pub bg_color: (u8, u8, u8, u8),
    pub is_cursor_blink: bool,
    pub is_wireframe: bool,
}

#[derive(Debug)]
pub struct Buffer {
    pub buffer_file_loc: Option<String>,
    pub buffer_cells: Vec<Vec<BufferCell>>,
}

impl Buffer {

    /// creates an empty buffer with no location or data
    pub fn new() -> Self {
        Buffer{
            buffer_file_loc: None,
            buffer_cells: Vec::new(),
        }
    }

    /// loads buffer_arr with data from source of buffer
    pub fn read() -> Self {
        Buffer{
            buffer_file_loc: None,
            buffer_cells: Vec::new(),
        }
    }

    /// writes buffer_arr to file location buffer_loc
    pub fn write() -> Self {
        Buffer{
            buffer_file_loc: None,
            buffer_cells: Vec::new(),
        }
    }
}
