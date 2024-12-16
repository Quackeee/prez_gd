use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};

use crate::objects::app_state::AppState;
use crate::objects::component::Component;


use super::component::ComponentLogic;
use super::transform::{Transform, TransformData};

pub struct GameObject {
    is_enabled: bool,
    transform_data: TransformData,
    components: Vec<Rc<RefCell<Component>>>,
    parent: Option<Weak<RefCell<GameObject>>>,
    self_reference: Option<Weak<RefCell<GameObject>>>,
    children: Vec<Rc<RefCell<GameObject>>>,
}

#[allow(dead_code)]
impl GameObject {
    pub fn start(&mut self, state: &AppState) {
        if !self.is_enabled {
            return;
        }

        for component in self.components.iter_mut() {
            component.borrow_mut().start(state);
        }

        for child in self.children.iter_mut() {
            child.borrow_mut().start(state);
        }
    }

    pub fn update(&mut self, state: &AppState) {
        if !self.is_enabled {
            return;
        }

        for component in self.components.iter_mut() {
            component.borrow_mut().update(state);
        }

        for child in self.children.iter_mut() {
            child.borrow_mut().update(state);
        }
    }

    pub fn draw(&self, context: &web_sys::WebGl2RenderingContext) {
        if !self.is_enabled {
            return;
        }

        for component in self.components.iter() {
            component.borrow().draw(context);
        }

        for child in self.children.iter() {
            child.borrow().draw(context);
        }
    }

    pub(in crate::objects) fn set_as_root_node(&mut self, reference: Rc<RefCell<GameObject>>) {
        self.self_reference = Some(Rc::downgrade(&reference));
    }

    pub fn new() -> Rc<RefCell<Self>> {
        let new_object = Rc::new(RefCell::new(Self {
            components: vec![],
            transform_data: TransformData::new(),
            is_enabled: true,
            self_reference: None,
            parent: None,
            children: vec![]
        }));
        new_object
    }

    pub fn add_component<T: ComponentLogic + 'static>(&mut self, logic: T) {
        let new_ref = self.self_reference.as_ref().expect("Self reference not found. Object was not Instantiated!").clone();
        self.components.push(Component::new_rc(Box::new(logic), new_ref));
    }

    pub fn add_child(&mut self, child: Rc<RefCell<GameObject>>) -> Rc<RefCell<GameObject>> {
        let child = child.clone();
        child.borrow_mut().parent = self.self_reference.clone();
        child.borrow_mut().self_reference = Some(Rc::downgrade(&child));
        self.children.push(child.clone());
        child
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn get_parent(&self) -> Option<Weak<RefCell<GameObject>>> {
        self.parent.clone()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

impl Transform for GameObject {
    fn get_data(&self) -> &TransformData {
        &self.transform_data
    }

    fn get_data_mut(&mut self) -> &mut TransformData {
        &mut self.transform_data
    }

    fn get_parent(&self) -> Option<Weak<RefCell<Self>>> {
        self.parent.clone()
    }

    fn get_children(&self) -> Vec<Weak<RefCell<Self>>> {
        self.children.iter().map(|child| child.borrow().self_reference.clone().unwrap()).collect()
    }
}