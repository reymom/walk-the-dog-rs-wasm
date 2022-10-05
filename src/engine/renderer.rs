use crate::engine::{Point, Rect, Renderer};
use web_sys::HtmlImageElement;

impl Renderer {
    pub fn clear(&self, rect: &Rect) {
        self.context.clear_rect(
            rect.x().into(),
            rect.y().into(),
            rect.width.into(),
            rect.height.into(),
        )
    }

    pub fn draw_image(&self, image: &HtmlImageElement, frame: &Rect, destination: &Rect) {
        self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &image,
                frame.x().into(),
                frame.y().into(),
                frame.width.into(),
                frame.height.into(),
                destination.x().into(),
                destination.y().into(),
                destination.width.into(),
                destination.height.into(),
            )
            .expect("Drawing is launching unrecoverable errors");
    }

    pub fn draw_entire_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    // pub fn draw_rect(&self, rect: &Rect) {
    //     self.context.set_stroke_style(&JsValue::from_str("#FF5000"));
    //     self.context.begin_path();
    //     self.context.rect(
    //         rect.x().into(),
    //         rect.y().into(),
    //         rect.width.into(),
    //         rect.height.into(),
    //     );
    //     self.context.stroke();
    // }
}
