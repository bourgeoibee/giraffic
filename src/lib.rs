use std::io::prelude::*;

pub enum GraphicError {
    OutOfBounds(usize),
}

#[derive(Debug)]
pub struct Pixels {
    pub width: usize,
    pub height: usize,
    pub contents: Vec<u32>,
}

impl Pixels {
    pub fn new(width: usize, height: usize) -> Self {
        let mut contents = Vec::new();
        contents.resize(width * height, 0);

        Pixels {
            width,
            height,
            contents,
        }
    }

    pub fn fill(self: &mut Self, color: u32) {
        for p in self.contents.iter_mut() {
            *p = color;
        }
    }

    pub fn fill_rect(
        self: &mut Self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) -> Result<(), GraphicError> {
        for i in y..y + height {
            for j in x..x + width {
                let ptr = match self.contents.get_mut(i * self.width + j) {
                    Some(p) => p,
                    None => return Err(GraphicError::OutOfBounds(i * self.width + j)),
                };

                *ptr = color;
            }
        }

        Ok(())
    }

    pub fn save_to_ppm(self: Self, path: &str) {
        let mut buffer = String::new();

        buffer.push_str(&format!(
            "P3\n{} {}\n255\n",
            self.width,
            self.contents.len() / self.width
        ));

        for p in self.contents {
            let red = p >> 3 * 8;
            let green = (p >> 2 * 8) & 0xFF;
            let blue = (p >> 1 * 8) & 0xFF;

            let line = &format!("{} {} {}\n", red, green, blue);
            buffer.push_str(line);
        }

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to open file");

        file.write_all(buffer.as_bytes())
            .expect("Failed to write to file");
    }
}
