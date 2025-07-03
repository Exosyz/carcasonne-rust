pub trait Renderer {
    fn render(&self, text: &str);
}

pub trait UIComponent {
    fn render(&self, renderer: &dyn Renderer);
}
