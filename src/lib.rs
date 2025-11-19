extern crate console_error_panic_hook;
extern crate wasm_bindgen;
extern crate web_sys;
use image::{DynamicImage, GenericImage, ImageBuffer, ImageEncoder, ImageReader, Rgba};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};

fn get_image_details() -> Result<(u32, u32, u32, u32), JsError> {
    let window = web_sys::window().ok_or(JsError::new("Can't get window"))?;
    let document = window
        .document()
        .ok_or(JsError::new("Can't get document"))?;
    let img: web_sys::HtmlImageElement = document
        .get_element_by_id("cropper")
        .ok_or(JsError::new("image not found in dom"))?
        .dyn_into()
        .map_err(|_| JsError::new("is not an image element"))?;
    let img_bounds = img.get_bounding_client_rect();

    let wrapper_bounds = document
        .get_element_by_id("image-wrapper")
        .ok_or(JsError::new("image wrapper not found in dom"))?
        .get_bounding_client_rect();

    let scale_element: HtmlInputElement = document
        .get_element_by_id("zoom")
        .ok_or(JsError::new("zoom element not found in dom"))?
        .dyn_into()
        .map_err(|_| JsError::new("zoom element is not an input element"))?;
    let scale = scale_element.value_as_number();

    let x = wrapper_bounds.left() - img_bounds.left();
    let y = wrapper_bounds.top() - img_bounds.top();

    let cw = f64::from(img.client_width());
    let ch = f64::from(img.client_height());
    let iw = f64::from(img.natural_width()) / scale;
    let ih = f64::from(img.natural_height()) / scale;
    let px = (x / cw) * iw;
    let py = (y / ch) * ih;

    let width = ((x + wrapper_bounds.width()) / cw) * iw - px;
    let height = ((y + wrapper_bounds.height()) / ch) * ih - py;

    let px = px as i32;
    let py = py as i32;
    let width = width as i32;
    let height = height as i32;
    let px = u32::try_from(px).map_err(|_| JsError::new("px is not positive"))?;
    let py = u32::try_from(py).map_err(|_| JsError::new("py is not positive"))?;
    let width = u32::try_from(width).map_err(|_| JsError::new("Bild füllt nicht den Rahmen"))?;
    let height = u32::try_from(height).map_err(|_| JsError::new("Bild füllt nicht den Rahmen"))?;
    return Ok((px, py, width, height));
}

#[wasm_bindgen]
pub fn gen(input_image: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let (px, py, width, height) = get_image_details()?;

    let reader = ImageReader::new(Cursor::new(input_image))
        .with_guessed_format()
        .map_err(|e| JsError::new(&format!("Can't read image: {:?}", e)))?;

    let image = reader
        .decode()
        .map_err(|e| JsError::new(&format!("Can't decode image: {:?}", e)))?;

    let image = image.crop_imm(px, py, width, height);

    let result = generate_passphoto(image)
        .map_err(|e| JsError::new(&format!("Can't generate passphoto: {:?}", e)))?;

    let mut result_vec = Vec::new();
    let encode = image::codecs::png::PngEncoder::new(&mut result_vec);
    encode
        .write_image(
            &result,
            result.width(),
            result.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| JsError::new(&format!("Can't encode image {:?}", e)))?;
    return Ok(result_vec);
}

fn generate_passphoto(
    img: DynamicImage,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, image::ImageError> {
    let width_of_result_in_px = 3543;
    let height_of_result_in_px = 2362;

    let width_of_result_in_mm = 150;
    let scale = width_of_result_in_px / width_of_result_in_mm;

    let width_of_crop = 35 * scale;
    let height_of_crop = 45 * scale;

    let cropped = img.resize(
        width_of_crop,
        height_of_crop,
        image::imageops::FilterType::Lanczos3,
    );

    let height_of_crop = cropped.height();

    let cropped = cropped.rotate90();

    let mut result_image =
        <ImageBuffer<Rgba<u8>, _>>::new(width_of_result_in_px, height_of_result_in_px);

    // should be 3mm
    let offset_x = (width_of_result_in_px - (3 * height_of_crop)) / 4;
    let offset_y = offset_x;

    for x in 0..3 {
        for y in 0..2 {
            result_image.copy_from(
                &cropped,
                (x + 1) * offset_x + x * cropped.width(),
                (y + 1) * offset_y + y * cropped.height(),
            )?;
        }
    }
    return Ok(result_image);
}
