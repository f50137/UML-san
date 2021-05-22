use std::io;
use std::fs;
use std::cmp;

mod java;
use java::*;

mod parser;
use parser::Parser;

mod image;
use image::Image;

use raylib::prelude::*;
use freetype::face;
use freetype::Library;

const WINDOW_WIDTH: i32 = 720;
const WINDOW_HEIGHT: i32 = 1280;
const FONT_SIZE: u32 = 40;
const UML_PADDING: u32 = 20;
const UML_ITEM_HEIGHT: u32 = UML_PADDING * 2 + FONT_SIZE;

fn measure_class(class: &Class, font: &face::Face) -> (u32, u32) {
    let field_strs = class.fields.iter().map(|f| f.to_string()).collect::<Vec<_>>();
    let method_strs = class.methods.iter().map(|f| f.to_string()).collect::<Vec<_>>();

    let max_field_text_width = field_strs.iter()
                                         .map(|f| Image::measure_text(f, FONT_SIZE as usize, font))
                                         .max().unwrap_or(0);
    
    let max_method_text_width = method_strs.iter()
                                           .map(|f| Image::measure_text(f, FONT_SIZE as usize, font))
                                           .max().unwrap_or(0);

    let class_name_width = Image::measure_text(&class.name, FONT_SIZE as usize, font);

    let uml_width = cmp::max(class_name_width, cmp::max(max_field_text_width, max_method_text_width)) + UML_PADDING * 2;
    let uml_height = (field_strs.len() + method_strs.len() + 1) as u32 * UML_ITEM_HEIGHT;

    (uml_width, uml_height)
}

fn render_class_new(img: &mut Image, class: &Class, start_x: u32, start_y: u32, font: &face::Face) {
    let field_strs = class.fields.iter().map(|f| f.to_string()).collect::<Vec<_>>();
    let method_strs = class.methods.iter().map(|f| f.to_string()).collect::<Vec<_>>();

    let max_field_text_width = field_strs.iter()
                                         .map(|f| Image::measure_text(f, FONT_SIZE as usize, font))
                                         .max().unwrap_or(0);
    
    let max_method_text_width = method_strs.iter()
                                           .map(|f| Image::measure_text(f, FONT_SIZE as usize, font))
                                           .max().unwrap_or(0);

    let class_name_width = Image::measure_text(&class.name, FONT_SIZE as usize, font);

    let uml_width = cmp::max(class_name_width, cmp::max(max_field_text_width, max_method_text_width)) + UML_PADDING * 2;
    let uml_height = (field_strs.len() + method_strs.len() + 1) as u32 * UML_ITEM_HEIGHT;

    img.render_rect(start_x, start_y, uml_width, uml_height, 0);
    img.render_text(start_x + uml_width / 2 - class_name_width / 2, start_y + UML_PADDING,
                    &class.name, FONT_SIZE as usize, font);

    img.render_horizontal_line(start_y + UML_ITEM_HEIGHT, start_x, start_x + uml_width, 0);

    for (i, text) in field_strs.iter().enumerate() {
        img.render_text(start_x + UML_PADDING, start_y + (i as u32 + 1) * UML_ITEM_HEIGHT + UML_PADDING, text,
                        FONT_SIZE as usize, font);
    }
    
    let method_start_y = start_y + UML_ITEM_HEIGHT * (field_strs.len() as u32 + 1);
    img.render_horizontal_line(method_start_y, start_x, start_x + uml_width, 0);

    for (i, text) in method_strs.iter().enumerate() {
        img.render_text(start_x + UML_PADDING, method_start_y + (i as u32) * UML_ITEM_HEIGHT + UML_PADDING,
                        text, FONT_SIZE as usize, font);
    }
}

fn main() -> io::Result<()> {
    let source_file = if let Some(name) = std::env::args().nth(1) {
        name
    } else {
        "sample.java".to_string()
    };
    
    let font_name = if let Some(name) = std::env::args().nth(2) {
        name
    } else {
        "font.ttf".to_string()
    };

    let source = fs::read_to_string(&source_file)?;
    let mut parser = Parser::new(&source);
    let class = parser.parse_class_def();

    if parser.diagnostics.len() > 0 {
        for d in parser.diagnostics {
            println!("(pos {}) Parsing Error: {}", d.pos, d.message);
        }

        std::process::exit(1);
    }

    let class = class.unwrap();

    set_trace_log(TraceLogType::LOG_NONE);
    let (mut rl, thread) = raylib::init().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("UML-san uwu").build();
    rl.set_target_fps(60);
    rl.set_window_state(rl.get_window_state().set_window_resizable(true));

    let lib = Library::init().unwrap();
    let face = lib.new_face(&font_name, 0).unwrap();

    const CLASS_PADDING: u32 = 2;
    const DOUBLE_PAD: u32 = CLASS_PADDING * 2 + 1;
    
    let (class_width, class_height) = measure_class(&class, &face);

    let mut img = Image::new(class_width as usize + DOUBLE_PAD as usize, class_height as usize + DOUBLE_PAD as usize);
    render_class_new(&mut img, &class, CLASS_PADDING, CLASS_PADDING, &face);
    img.save_to_file_ppm("save.ppm")?;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        for x in 0..class_width + DOUBLE_PAD {
            for y in 0..class_height + DOUBLE_PAD {
                let grey_val = img.get(x as u32, y as u32);
                d.draw_pixel(x as i32, y as i32, Color::new(grey_val, grey_val, grey_val, 255))
            }
        }
    }

    Ok(())
}
