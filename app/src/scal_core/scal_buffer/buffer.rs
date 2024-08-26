use std::io::BufRead;

use crate::scal_core::scal_buffer::buffer_cell::BufferCell;

#[derive(Debug)]
pub struct Buffer {
    pub buffer_file_loc: Option<String>,
    pub buffer_cells: Vec<Vec<BufferCell>>,
}

impl Buffer {

    pub fn new_test_one() -> Self {

        let mut buffer = Buffer::new();

        let line1_text = "This is a scalic text editor";
        let line2_text = "this is a monkas test";
        let line3_text = "this is a test of the emergency broadcast system";

        let mut line1 = Vec::new();
        let mut line2 = Vec::new();
        let mut line3 = Vec::new();

        for c in line1_text.chars() {
            line1.push(BufferCell{
                character: c,
                fg_color: (255, 255, 255, 255),
                bg_color: (0, 0, 0, 255),
                is_cursor_blink: false,
                is_wireframe: false,
            });
        }

        for c in line2_text.chars() {
            line2.push(BufferCell{
                character: c,
                fg_color: (255, 255, 255, 255),
                bg_color: (255, 0, 0, 255),
                is_cursor_blink: false,
                is_wireframe: false,
            });
        }

        for c in line3_text.chars() {
            line3.push(BufferCell{
                character: c,
                fg_color: (255, 255, 255, 255),
                bg_color: (0, 0, 0, 255),
                is_cursor_blink: false,
                is_wireframe: false,
            });
        }

        buffer.buffer_cells.push(line1);
        buffer.buffer_cells.push(line2);
        buffer.buffer_cells.push(line3);

        buffer
    }

    /// creates an empty buffer with no location or data
    pub fn new() -> Self {
        Buffer{
            buffer_file_loc: None,
            buffer_cells: Vec::new(),
        }
    }

    /// loads buffer_arr with data from source of buffer
    pub fn read(file_path: &str) -> anyhow::Result<Self> {

        let mut buffer = Buffer::new();

        for line in std::fs::read_to_string(file_path).unwrap().lines() {
            let mut line_vec = Vec::new();
            for c in line.chars() {
                line_vec.push(BufferCell{
                    character: c,
                    fg_color: (255, 255, 255, 255),
                    bg_color: (0, 0, 0, 255),
                    is_cursor_blink: false,
                    is_wireframe: false,
                });
            }
            buffer.buffer_cells.push(line_vec);
        }

        Ok(buffer)
    }

    /// writes buffer_arr to file location buffer_loc
    pub fn write() -> anyhow::Result<Self> {
        Ok(Buffer::new())
    }
}
