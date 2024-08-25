
use crate::scal_buffer;

/// A ScalSession is a struct that holds the current buffer being edited and a list of open buffers. 
/// current_buffer is the index of the buffer in open_buffers that is currently being edited.
/// open_buffers is a list of all buffers that are currently open in the editor.
pub struct ScalSession {
    pub current_buffer: i32,
    pub open_buffers: Vec<scal_buffer::Buffer>,
}

impl ScalSession{
    pub fn new() -> Self {
        ScalSession{
            current_buffer: -1,
            open_buffers: Vec::new()
        }
    }
}

