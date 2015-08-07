use std::fs::File;
use std::io::{Write, self};
use layout::{LayoutBox, Rect};
use painting::{DisplayCommand, build_display_list};

extern crate pdf;
use self::pdf::{Pdf, Canvas};


fn px_to_pt(value: f32) -> f32 {
    // 96px = 1in = 72pt
    // value * 1px = value * 96px / 96 = value * 72pt / 96 = (value * 0.75) * 1pt
    value * 0.75
}


pub fn render(layout_root: &LayoutBox, bounds: Rect, file: &mut File) -> io::Result<()> {
    let display_list = build_display_list(layout_root);
    let mut pdf = try!(Pdf::new(file));
    // We map CSS pt to Poscript points (which is the default length unit in PDF).
    try!(pdf.render_page(px_to_pt(bounds.width), px_to_pt(bounds.height), |canvas| {
        for item in display_list {
            try!(render_item(&item, canvas));
        }
        Ok(())
    }));
    pdf.finish()
}


fn render_item<W: Write>(item: &DisplayCommand, canvas: &mut Canvas<W>) -> io::Result<()> {
    match *item {
        DisplayCommand::SolidColor(color, rect) => {
            canvas.rectangle(
               // FIMXE: alpha transparency
               color.r, color.g, color.b,
               rect.x, rect.y, rect.width, rect.height)
        }
    }
}
