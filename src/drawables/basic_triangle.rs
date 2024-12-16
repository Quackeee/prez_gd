use std::{cell::RefCell, rc::Weak};

use web_sys::WebGl2RenderingContext;

use crate::objects::component::{Component, ComponentLogic};

pub struct BasicTriangle;

impl ComponentLogic for BasicTriangle {
    fn draw(&self,_component: Weak<RefCell<Component>>, context: &WebGl2RenderingContext) {
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0,  3);
    }
}

impl BasicTriangle {
    pub fn new() -> Self {
        Self {}
    }
}