use crate::*;
use image::{Rgba, RgbaImage};

// TODO: Support any pixel type
pub trait Renderable<I>
where
    Self: Sized + Plotable<I, i64>,
    I: Iterator<Item = Point<i64>>,
{
    fn render(self, image_buffer: &mut RgbaImage) {
        self.plot().for_each(|Point(x, y)| {
            image_buffer.put_pixel(x as u32, y as u32, Rgba([0, 0, 0, 255]))
        })
    }

    fn render_with(self, image_buffer: &mut RgbaImage, draw_fn: impl Fn((u32, u32)) -> Rgba<u8>) {
        self.plot().for_each(|Point(x, y)| {
            let (x, y) = (x as u32, y as u32);

            image_buffer.put_pixel(x, y, draw_fn((x, y)))
        })
    }
}

impl Renderable<LinePlot> for Line {}
impl Renderable<PolyLinePlot> for PolyLine {}
