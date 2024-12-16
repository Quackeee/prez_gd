use std::panic;

use drawables::basic_background::BasicBackground;
use drawables::basic_triangle::BasicTriangle;
use objects::game_object::GameObject;
use wasm_bindgen::prelude::*;
use web_sys::{window, WebGl2RenderingContext, WebGlProgram, WebGlShader};

mod console;
extern crate nalgebra_glm as glm;

use objects::app_state::AppState;

mod drawables;
mod objects;
mod renderer;
mod utils;
mod input;

static mut WINDOW_ANIMATION_FRAME_REQUEST_CLOSURE: Option<Closure<dyn FnMut()>> = None;

#[wasm_bindgen(raw_module="/asset-utils.js")]
extern "C" {
    #[wasm_bindgen(js_name=getAsset)]
    async fn get_asset(name: &str) -> JsValue;
}

fn get_canvas() -> web_sys::HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .expect("No canvas with id 'canvas'");
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Element with id 'canvas' is not a canvas")
}

fn get_window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

#[wasm_bindgen(start)]
async fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(|info| {
        // This is added to be able to set a breakpoint in the debugger
        let info = info;
        console::error!("{}", info);
    }));
    console::log!("Hello from Rust!");

    let canvas = get_canvas();

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    console::log!("Compiling vertex shader...");
        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            get_asset("shaders/triangle.vert").await.as_string().unwrap().as_str()
        )?;

    console::log!("Compiling fragment shader...");
        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            get_asset("shaders/triangle.frag").await.as_string().unwrap().as_str()
        )?;

    console::log!("Linking program...");
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    console::log!("Shader initialization complete!");

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        drop(positions_array_buf_view);
    }

    console::log!("Binding vertex array object...");
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    console::log!("Dispatching render loop...");
    let time = window().unwrap().performance().unwrap().now() as f32 / 1000.0;
    let mut state = AppState::new(context, time);

    state.add_object_empy().borrow_mut().add_component(BasicBackground::new());

    let triangle = GameObject::new();
    triangle.borrow_mut().set_enabled(true);
    state.add_object(triangle).borrow_mut().add_component(BasicTriangle::new());

    console::log!("Registering callbacks...");

    state.setup_callbacks(&canvas);

    render_loop_event(state);

    console::log!("Rust initalization complete!");

    Ok(())
}

fn on_canvas_resize(old_width: u32, old_height: u32, new_width: u32, new_height: u32) {
    console::log!(
        "Resizing from {}x{} to {}x{}",
        old_width,
        old_height,
        new_width,
        new_height
    );
}

fn draw(state: &mut AppState, delta_time: f32) {
    let _ = delta_time;

    state.draw();
}

#[allow(static_mut_refs)]
fn render_loop_event(mut state: AppState) {
    let canvas = get_canvas();
    if canvas.client_width() as u32 != canvas.width()
        || canvas.client_height() as u32 != canvas.height()
    {
        let old_width = canvas.width();
        let old_height = canvas.height();
        let new_width = canvas.client_width() as u32;
        let new_height = canvas.client_height() as u32;
        canvas.set_width(new_width);
        canvas.set_height(new_height);
        on_canvas_resize(old_width, old_height, new_width, new_height);
    }

    state.process_events();

    if state.keyboard.was_key_pressed(65 /* A */) {
        console::log!("A key pressed");
    }
    if state.mouse.was_button_pressed(0) {
        console::log!("LMB pressed")
    }
    if state.mouse.was_button_pressed(1) {
        console::log!("MMB pressed")
    }
    if state.mouse.was_button_pressed(2) {
        console::log!("RMB pressed")
    }
    if state.mouse.mouse_delta.0 != 0 || state.mouse.mouse_delta.1 != 0 {
        console::log!("Mouse position: {}, {}", state.mouse.position.0, state.mouse.position.1)
    }
    if state.mouse.scroll_delta != 0.0 {
        console::log!("Scroll delta: {}", state.mouse.scroll_delta) 
    }

    let time = window().unwrap().performance().unwrap().now() as f32 / 1000.0;
    state.update(time);

    let delta_time = state.time.delta_time;
    draw(&mut state, delta_time);

    let a = Closure::once(move || {
        render_loop_event(state);
    });

    get_window()
        .request_animation_frame(a.as_ref().unchecked_ref())
        .unwrap();

    unsafe {
        WINDOW_ANIMATION_FRAME_REQUEST_CLOSURE.replace(a);
    }
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
