use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::utils::Point;

pub struct Image {
    filename: String,
    image: HtmlImageElement,
    loc: Point<f64>
}

impl Image {
    pub fn new(filename: String, x: f64, y: f64) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(&filename.clone());
        // image.set_src(&li.filename.replace("./","/"));

        Image {
            filename: filename,
            image: image,
            loc: Point::new(x, y)
        }
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.translate(self.loc.x, self.loc.y);
        let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &self.image, 
            0.0,
            0.0,
            self.image.width() as f64,
            self.image.height() as f64,
            -(self.image.width() as f64 / 2.0),
            -(self.image.height() as f64 / 2.0),
            self.image.width() as f64,
            self.image.height() as f64
        );
        let _ = ctx.translate(-self.loc.x, -self.loc.y);
    }
}