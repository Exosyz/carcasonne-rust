use crate::renderable::container_renderer::ContainerDirection;
use crate::renderable::Renderable;

pub trait RenderableProps {
    fn has_full_size_in_direction(&self, dir: ContainerDirection) -> bool;
}

impl<'a> RenderableProps for dyn Renderable + 'a {
    fn has_full_size_in_direction(&self, dir: ContainerDirection) -> bool {
        if let Some(container) = self.as_container() {
            container.has_full_size_in_direction(dir)
        } else {
            false
        }
    }
}
