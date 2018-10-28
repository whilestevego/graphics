use image::{Rgba, RgbaImage};
use line::*;
use plotable::*;
use point::*;
use poly_line::*;

pub trait Renderable<I>
where
    Self: Sized + Plotable<I>,
    I: Iterator<Item = Point>,
{
    fn render(self, image_buffer: &mut RgbaImage) {
        self.plot().for_each(|Point { x, y }| {
            image_buffer.put_pixel(x.round() as u32, y.round() as u32, Rgba([0, 0, 0, 255]))
        })
    }

    fn render_with(self, image_buffer: &mut RgbaImage, draw_fn: impl Fn((u32, u32)) -> Rgba<u8>) {
        self.plot().for_each(|Point { x, y }| {
            let (x, y) = (x.round() as u32, y.round() as u32);

            image_buffer.put_pixel(x, y, draw_fn((x, y)))
        })
    }
}

impl Renderable<LinePlot> for Line {}
impl Renderable<PolyLinePlot> for PolyLine {}