use wasm_bindgen::{prelude::*, Clamped, JsCast};
use wasm_bindgen;
use web_sys::console;
use web_sys::ImageData;
use num::Complex;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn render_fractal(scale: f32, c_0_r: f32, c_0_i: f32, r_grad: f32, b_grad: f32) {
    
    console::log_1(&"Entering render".into());

    let window = web_sys::window().unwrap();
    let document = window.document().expect("Could not get document");
    let canvas = document
        .get_element_by_id("myCanvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let canvas_dims = format!("Canvas is {}x{} (hxw)", canvas.height(), canvas.width());
    console::log_1(&canvas_dims.into());

    let imgx = canvas.width();
    let imgy = canvas.height();

    let scalex = scale / imgx as f32;
    let scaley = scale / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (r_grad * x as f32) as u8;
        let b = (b_grad * y as f32) as u8;
        let cx = y as f32 * scalex - 1.5;
        let cy = x as f32 * scaley - 1.5;

        let c = num_complex::Complex::new(c_0_r, c_0_i);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }

        *pixel = image::Rgba([r, i as u8, b, 255]);
    }

    let context = canvas
        .get_context("2d")
        .unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    let clamped_buf: Clamped<&[u8]> = Clamped(imgbuf.as_raw());
    let image_data_temp = 
        ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, imgbuf.width(), imgbuf.height()).unwrap();
    _ = context.put_image_data(&image_data_temp, 0.0, 0.0);

}
