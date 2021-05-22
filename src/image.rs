use freetype::face;
use freetype::bitmap::PixelMode;
use std::io;

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {

        Self {
            width,
            height,
            data: vec![255; width * height],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        let y = y as usize;
        let x = x as usize;
        if x < self.width && y < self.height {
            self.data[x + y * self.width]
        } else {
            0
        }
    }

    fn set(&mut self, x: u32, y: u32, val: u8) {
        let y = y as usize;
        let x = x as usize;
        if x < self.width && y < self.height {
            self.data[x + y * self.width] = val;
        }
    }

    pub fn render_text(&mut self, x: u32, y: u32, text: &str, font_size: usize, font: &face::Face) {
        
        let mut pen_x = x as i32;
        let mut pen_y = y as i32;

        font.set_char_size(0, font_size as isize * 64, 0, 0)
            .expect("Could not set the font size!");
        

        for c in text.chars() {
            font.load_char(c as usize, face::LoadFlag::RENDER)
                .expect(&format!("Could not load char {:?} in text {:?}!", c, text));

            let glyph = font.glyph();
            let bitmap = glyph.bitmap();

            //TODO: support multiple pixel modes
            assert!(bitmap.pixel_mode().unwrap() == PixelMode::Gray);

            let char_start_x = pen_x + glyph.bitmap_left();
            let char_start_y = pen_y + (font_size as i32 - glyph.bitmap_top());

            for y in 0..bitmap.rows() {
                for x in 0..bitmap.width() {
                    let grey_val = bitmap.buffer()[(x + y * bitmap.pitch()) as usize];
                    let pos_x = char_start_x + x;
                    let pos_y = char_start_y + y;

                    if 0 <= pos_x && pos_x < self.width as i32 && 0 <= pos_y && pos_y < self.height as i32 {
                        self.set(pos_x as u32, pos_y as u32, 255 - grey_val);
                    }
                }
            }

            pen_x += (glyph.advance().x >> 6) as i32;
            pen_y += (glyph.advance().y >> 6) as i32;
        }
    }

    pub fn measure_text(text: &str, font_size: usize, font: &face::Face) -> u32 {
        let mut width: u32 = 0;

        font.set_char_size(0, font_size as isize * 64, 0, 0).expect("Could not set the font size!");

        for c in text.chars() {
            font.load_char(c as usize, face::LoadFlag::RENDER)
                .expect(&format!("Could not load char {:?} in text {:?}!", c, text));

            let glyph = font.glyph();
            width += (glyph.advance().x >> 6) as u32;
        }

        width
    }

    pub fn render_horizontal_line(&mut self, y: u32, x1: u32, x2: u32, grey_val: u8) {
        for x in x1..=x2 {
            self.set(x, y, grey_val);
        }
    }
    
    pub fn render_vertical_line(&mut self, x: u32, y1: u32, y2: u32, grey_val: u8) {
        for y in y1..=y2 {
            self.set(x, y, grey_val);
        }
    }

    pub fn render_rect(&mut self, x: u32, y: u32, w: u32, h: u32, grey_val: u8) {
        self.render_horizontal_line(y,     x, x + w, grey_val);
        self.render_horizontal_line(y + h, x, x + w, grey_val);

        self.render_vertical_line(x, y, y + h, grey_val);
        self.render_vertical_line(x + w, y, y + h, grey_val);
    }

    pub fn save_to_file_ppm(&self, file_path: &str) -> io::Result<()> {
        let mut ppm_string = String::from("");

        ppm_string.push_str("P2\n");
        ppm_string.push_str(&format!("{} {}\n", self.width, self.height));
        ppm_string.push_str(&format!("{}\n", 255));

        for y in 0..self.height {
            for x in 0..self.width {
                ppm_string.push_str(&format!("{} ", self.get(x as u32, y as u32)));
            }
            ppm_string.push('\n');
        }

        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(file_path)?;
        file.write_all(ppm_string.as_bytes())?;
        Ok(())
    }
}
