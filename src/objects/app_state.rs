use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext, WheelEvent};

use crate::objects::game_object::GameObject;
use crate::renderer::gl_render::GLRender;

use crate::input::{InputEvent, KeyboardState, KeyboardStateSnapshot, MouseState, MouseStateSnapshot};

pub struct Time {
    start_time: f32,
    last_time: f32,
    pub delta_time: f32,
    pub elapsed_time: f32,
}

impl Time {
    pub fn new(now: f32) -> Self {
        Self {
            start_time: now,
            last_time: 0.0,
            delta_time: 0.0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, time: f32) {
        self.delta_time = time - self.last_time;
        self.elapsed_time = time - self.start_time;
        self.last_time = time;
    }
}

pub struct AppState {
    pub time: Time,
    events: Arc<Mutex<Vec<InputEvent>>>,
    renderer: GLRender,
    root_object: Rc<RefCell<GameObject>>,
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    pub keyboard: KeyboardStateSnapshot,
    pub mouse: MouseStateSnapshot
}

impl AppState {
    pub fn new(context: WebGl2RenderingContext, cur_time: f32) -> Self {
        let root_node = GameObject::new();
        root_node.borrow_mut().set_as_root_node(root_node.clone());

        let keyboard_state = KeyboardState::new();
        let keyboard_state_snapshot = KeyboardStateSnapshot::from(&keyboard_state);

        let mouse_state = MouseState::new();
        let mouse_state_snapshot = MouseStateSnapshot::from(&mouse_state);

        Self {
            events: Arc::from(Mutex::from(Vec::new())),
            renderer: GLRender::new(context),
            time: Time::new(cur_time),
            root_object: root_node,
            keyboard_state,
            mouse_state: MouseState::new(),
            keyboard: keyboard_state_snapshot,
            mouse: mouse_state_snapshot
        }
    }

    pub fn add_object_empy(&mut self) -> Rc<RefCell<GameObject>> {
        let new_child = self.root_object.borrow_mut().add_child(GameObject::new());
        new_child
    }

    pub fn add_object(&mut self, object: Rc<RefCell<GameObject>>) -> Rc<RefCell<GameObject>> {
        let new_child = self.root_object.borrow_mut().add_child(object);
        new_child
    }

    pub fn update(&mut self, cur_time: f32) {
        self.time.update(cur_time);
        let mut object = self.root_object.borrow_mut();
        // Start the object if it hasn't been started
        // Start is called only once and this is handled by GameObject internals.
        object.start(&self);
        object.update(&self);
    }

    pub fn setup_callbacks(&mut self, canvas: &HtmlCanvasElement) {
        /* mouse move */
        {
            let events = self.events.clone();
            let closure: Box<dyn FnMut(MouseEvent)> = Box::new(move |e: MouseEvent| {
                let mut events = events.lock().unwrap();
                events.push(InputEvent::MouseMoved(e.screen_x(), e.screen_y()));
            });

            let closure = Closure::wrap(closure);

            canvas.set_onmousemove(Some(closure.as_ref().unchecked_ref()));

            closure.forget();
        }

        /* mouse click */
        {
            let events = self.events.clone();
            let closure: Box<dyn FnMut(MouseEvent)> = Box::new(move |e: MouseEvent| {
                let mut events = events.lock().unwrap();
                events.push(InputEvent::MouseKeyPressed(e.screen_x(), e.screen_y(), e.button()));
            });

            let closure = Closure::wrap(closure);

            canvas.set_onmousedown(Some(closure.as_ref().unchecked_ref()));

            closure.forget();
        }

        /* mouse click release */
        {
            let events = self.events.clone();
            let closure: Box<dyn FnMut(MouseEvent)> = Box::new(move |e: MouseEvent| {
                let mut events = events.lock().unwrap();
                events.push(InputEvent::MouseKeyReleased(e.screen_x(), e.screen_y(), e.button()));
                // this will allow using mouse buttons that normally navigate through tab history
                e.prevent_default();
            });

            let closure = Closure::wrap(closure);

            canvas.set_onmouseup(Some(closure.as_ref().unchecked_ref()));

            closure.forget();
        }

        /* scroll */
        {
            let events = self.events.clone();
            let closure: Box<dyn FnMut(WheelEvent)> = Box::new(move |e: WheelEvent| {
                let mut events = events.lock().unwrap();
                events.push(InputEvent::ScrollMoved(e.delta_y()));
            });

            let closure = Closure::wrap(closure);

            canvas.set_onwheel(Some(closure.as_ref().unchecked_ref()));

            closure.forget();
        }

    }

    pub fn process_events(&mut self) {
        let mut events = self.events.lock().unwrap();

        self.keyboard_state.process_events(&events);
        self.mouse_state.process_events(&events);

        self.keyboard = self.keyboard_state.snapshot();
        self.mouse = self.mouse_state.snapshot();

        events.clear();
    }

    // TODO:
    // Limit drawing to only objects that implements draw() method
    // Maybe create a trait for drawable objects
    pub fn draw(&self) {
        self.renderer.render(self, &self.root_object);
    }
}
