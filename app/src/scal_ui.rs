pub trait View {
    fn render(&self);
}

pub struct LandingView;
pub struct BufferView;

impl View for LandingView {
    fn render(&self) {
        // render sdl2 text to the screen
    }
}

impl View for BufferView {
    fn render(&self) {
        println!("Buffer view");
    }
}

pub struct ScalUI {
    pub view: Box<dyn View>,
}

impl ScalUI {
    pub fn new() -> Self {
        ScalUI { view: Box::new(LandingView) }
    }

    pub fn render(&self) {
        self.view.render();
    }
}

