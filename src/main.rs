use giraffic::{Pixels, GraphicError};

fn triangle_area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
    let cross_product = 0.5 * ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1));
    cross_product.abs()
}

fn fill_triangle(
    pixels: &mut Pixels,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    x3: usize,
    y3: usize,
    color: u32,
) -> Result<(), GraphicError> {
    let startx = x1.min(x2).min(x3);
    let endx = x1.max(x2).max(x3);
    let starty = y1.min(y2).min(y3);
    let endy = y1.max(y2).max(y3);

    let total_area = triangle_area(
        x1 as f32, y1 as f32, x2 as f32, y2 as f32, x3 as f32, y3 as f32,
    );
    for y in starty..endy {
        for x in startx..endx {
            let area1 = triangle_area(
                x as f32, y as f32, x2 as f32, y2 as f32, x3 as f32, y3 as f32,
            );
            let area2 = triangle_area(
                x1 as f32, y1 as f32, x as f32, y as f32, x3 as f32, y3 as f32,
            );
            let area3 = triangle_area(
                x1 as f32, y1 as f32, x2 as f32, y2 as f32, x as f32, y as f32,
            );

            if total_area == area1 + area2 + area3 {
                let idx = y * pixels.width + x;
                let ptr = match pixels.contents.get_mut(idx) {
                    Some(p) => p,
                    None => return Err(GraphicError::OutOfBounds(idx)),
                };

                *ptr = color;
            }
        }
    }

    Ok(())
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const PATH: &str = "red.ppm";

    let red = 0xFF0000FF;
    let blue = 0xAA55EEFF;
    let green = 0x44DD66FF;
    let mut pixels = giraffic::Pixels::new(WIDTH, HEIGHT);

    match pixels.fill_rect(100, 200, 300, 400, red) {
        Err(giraffic::GraphicError::OutOfBounds(i)) => println!("Index out of bounds: {}", i),
        _ => {}
    }

    match pixels.fill_rect(300, 50, 200, 180, blue) {
        Err(giraffic::GraphicError::OutOfBounds(i)) => println!("Index out of bounds: {}", i),
        _ => {}
    }

    match fill_triangle(&mut pixels, 500, 50, 1000, 400, 300, 550, green) {
        Err(giraffic::GraphicError::OutOfBounds(i)) => println!("Index out of bounds: {}", i),
        _ => {}
    }
    pixels.save_to_ppm(PATH);
}
