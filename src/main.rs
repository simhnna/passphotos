use image::{GenericImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

fn main() {
    println!("Hello, world!");
    test().unwrap();
}

fn test() -> Result<image::DynamicImage, image::ImageError> {
    let width_of_result_in_px = 3543;
    let height_of_result_in_px = 2362;

    let width_of_result_in_mm = 150;
    let scale = width_of_result_in_px / width_of_result_in_mm;

    println!("Scale: {:?}", scale);
    let width_of_crop = 35 * scale;
    let height_of_crop = 45 * scale;
    println!(
        "Crop dimensions: {:?} x {:?}",
        width_of_crop, height_of_crop
    );

    let img = ImageReader::open("cropped.jpg")?.decode()?;
    println!(
        "Image dimensions: {:?} x {:?}",
        img.dimensions().0,
        img.dimensions().1
    );
    let cropped = img.resize(
        width_of_crop,
        height_of_crop,
        image::imageops::FilterType::Lanczos3,
    );

    let height_of_crop = cropped.height();
    let width_of_crop = cropped.width();

    let cropped = cropped.rotate90();
    println!(
        "Image dimensions: {:?} x {:?}",
        width_of_crop, height_of_crop
    );
    let mut result_image =
        <ImageBuffer<Rgba<u8>, _>>::new(width_of_result_in_px, height_of_result_in_px);

    // should be 3mm
    let offset_x = (width_of_result_in_px - (3 * height_of_crop)) / 4;
    let offset_y = offset_x;

    for x in 0..3 {
        for y in 0..2 {
            println!(
                "Copying image at: {:?} x {:?}",
                (x + 1) * offset_x + x * cropped.width(),
                (y + 1) * offset_y + y * cropped.height()
            );
            result_image.copy_from(
                &cropped,
                (x + 1) * offset_x + x * cropped.width(),
                (y + 1) * offset_y + y * cropped.height(),
            )?;
        }
    }

    result_image.save("result.png")?;
    return Ok(img);
}
