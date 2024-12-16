use std::cell::RefCell;

use web_sys::WebGl2RenderingContext;

use crate::objects::{app_state::AppState, game_object::GameObject};

pub struct GLRender {
    context: WebGl2RenderingContext,
}

impl GLRender {
    pub fn new(context: WebGl2RenderingContext) -> Self {
        Self {
            context
        }
    }

    pub fn render(&self, _state: &AppState, root_object: &RefCell<GameObject>) {
        let context = &self.context;
        context.viewport(0, 0, context.drawing_buffer_width(), context.drawing_buffer_height());
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        root_object.borrow().draw(context);
    }
}