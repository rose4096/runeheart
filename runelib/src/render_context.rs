use jni::sys::jlong;
use skia_safe::{
    AlphaType, Canvas, Color, ColorType, ISize, ImageInfo, Paint, PaintStyle, Surface, surfaces,
};

pub struct RenderContext {
    pub size: ISize,
    info: ImageInfo,
    surface: Surface,
    pub buffer: Vec<u8>,
}

impl RenderContext {
    pub fn from_handle(handle: jlong) -> &'static Self {
        unsafe { &*(handle as usize as *mut RenderContext) }
    }

    pub fn from_handle_mut(handle: jlong) -> &'static mut Self {
        unsafe { &mut *(handle as usize as *mut RenderContext) }
    }

    pub fn new(size: ISize) -> Self {
        let info = ImageInfo::new(size, ColorType::RGBA8888, AlphaType::Unpremul, None);
        let surface = surfaces::raster(&info, None, None).expect("surface");

        Self {
            size,
            info,
            surface,
            buffer: vec![0u8; (size.width * size.height * 4) as usize],
        }
    }

    pub fn resize_pixel_buffer(&mut self, size: ISize) {
        self.size = size;
        self.info = ImageInfo::new(size, ColorType::RGBA8888, AlphaType::Premul, None);
        self.surface = self
            .surface
            .new_surface(&self.info)
            .expect("surface resize");
        self.buffer = vec![0u8; (size.width * size.height * 4) as usize];
        self.fill_pixel_buffer();
    }

    pub fn fill_pixel_buffer(&mut self) {
        let rb = (self.size.width * 4) as usize;
        self.surface
            .read_pixels(&self.info, &mut self.buffer, rb, (0, 0));
    }

    pub fn canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }
}
