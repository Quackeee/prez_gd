use std::{cell::RefCell, rc::Weak};

use web_sys::WebGl2RenderingContext;

use crate::objects::component::{Component, ComponentLogic};

pub struct BasicBackground {
    r: f32,
    g: f32,
    b: f32
}

impl BasicBackground {
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
}

impl ComponentLogic for BasicBackground {
    fn update(&mut self, _component: Weak<RefCell<Component>>, state: &crate::objects::app_state::AppState) {
        if state.keyboard.is_key_pressed(40) /* down */ {
            self.r -= 0.01;
        }
        if state.keyboard.is_key_pressed(38) /* up */ {
            self.r += 0.01;
        }
        if state.keyboard.is_key_pressed(37) /* left */ {
            self.g -= 0.01;
        }
        if state.keyboard.is_key_pressed(39) /* right */ {
            self.g += 0.01;
        }
    }

    fn draw(&self,_component: Weak<RefCell<Component>>, context: &web_sys::WebGl2RenderingContext) {
        context.clear_color(self.r, self.g, self.b, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}