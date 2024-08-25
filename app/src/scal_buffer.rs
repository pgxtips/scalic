pub struct Buffer{
    pub buffer_arr: Vec<u8>,
    pub buffer_loc: String
}

impl Buffer{

    /// creates an empty buffer with no location or data
    pub fn new() -> Self {
        Buffer{
            buffer_arr: Vec::new(),
            buffer_loc: String::new()
        }
    }

    /// loads buffer_arr with data from source of buffer
    pub fn read() -> Self {
        Buffer{
            buffer_arr: Vec::new(),
            buffer_loc: String::new()
        }
    }

    /// writes buffer_arr to file location buffer_loc
    pub fn write() -> Self {
        Buffer{
            buffer_arr: Vec::new(),
            buffer_loc: String::new()
        }
    }
}
