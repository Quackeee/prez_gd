use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::objects::app_state::AppState;

use super::game_object::GameObject;

pub trait ComponentLogic {
    fn start(&mut self, _component: Weak<RefCell<Component>>, _state: &AppState) {}
    fn update(&mut self, _component: Weak<RefCell<Component>>, _state: &AppState) {}
    fn draw(&self, _component: Weak<RefCell<Component>>, _context: &web_sys::WebGl2RenderingContext) {}
}

pub struct Component {
    is_started: bool,
    is_enabled: bool,
    self_ptr: Weak<RefCell<Component>>,
    object: Weak<RefCell<GameObject>>,
    logic: Box<dyn ComponentLogic>
}

#[allow(dead_code)]
impl Component {
    pub(in crate::objects) fn start(&mut self, state: &AppState) {
        if !self.is_enabled {
            return;
        }

        if !self.is_started {
            self.logic.start(self.self_ptr.clone(), state);
            self.is_started = true;
        }
    }

    pub(in crate::objects) fn update(&mut self, state: &AppState) {
        if !self.is_enabled {
            return;
        }

        self.logic.update(self.self_ptr.clone(), state);
    }

    pub(in crate::objects) fn draw(&self, context: &web_sys::WebGl2RenderingContext) {
        if !self.is_enabled {
            return;
        }

        self.logic.draw(self.self_ptr.clone(), context);
    }

    pub(in crate::objects) fn new_rc(lgc: Box<dyn ComponentLogic>, object: Weak<RefCell<GameObject>>) -> Rc<RefCell<Self>> {
        let new_object = Rc::new(RefCell::new(
            Self {
                is_started: false,
                is_enabled: true,
                self_ptr: Weak::new(),
                object,
                logic: lgc,
            }
        ));
        new_object.borrow_mut().self_ptr = Rc::downgrade(&new_object);
        new_object
    }

    pub fn get_object(&self) -> Weak<RefCell<GameObject>> {
        self.object.clone()
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
    
    pub fn set_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }
}