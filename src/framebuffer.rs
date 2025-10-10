use raylib::prelude::*;
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    /// Clears the framebuffer by regenerating the color buffer with the background color
    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    /// Sets a single pixel in the buffer to the current color, if within bounds
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            // Calculate the offset into the Image data (raylib stores pixels in row-major order)
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
            //let offset = (y * self.width + x) as usize;
            // Safety: data is a contiguous Vec<u8> RGBA, but raylib-rs Image uses Color per u32
            //unsafe {
                //let ptr = self.color_buffer.data().as_mut_ptr() as *mut Color;
                //*ptr.add(offset) = self.current_color;
            //}
        }
    }
    pub fn get_color(&mut self, x: u32, y: u32) {
        self.color_buffer.get_color(x as i32, y as i32);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Exports the framebuffer to an image file (BMP/PNG/etc.) using raylib's FFI
    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}