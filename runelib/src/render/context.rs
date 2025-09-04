use crate::render::input::{Character, Delta, Input, KeyState, MouseButton, Position};
use crate::screen::ScreenRenderable;
use jni::JNIEnv;
use jni::objects::JByteBuffer;
use jni::sys::jlong;
use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};
use skia_safe::{
    AlphaType, Canvas, Color, ColorType, FontMgr, ISize, ImageInfo, Point, Surface, surfaces,
};

pub struct RenderContext {
    size: ISize,
    buffer: Vec<u8>,
    input: Input,

    // skia
    info: ImageInfo,
    surface: Surface,
    font_collection: FontCollection,
    renderables: Vec<Box<dyn ScreenRenderable>>,
}

impl RenderContext {
    pub fn from_handle(handle: jlong) -> &'static Self {
        unsafe { &*(handle as usize as *mut RenderContext) }
    }

    pub fn from_handle_mut(handle: jlong) -> &'static mut Self {
        unsafe { &mut *(handle as usize as *mut RenderContext) }
    }

    pub fn new(size: ISize) -> Self {
        let info = ImageInfo::new(size, ColorType::RGBA8888, AlphaType::Premul, None);
        let surface = surfaces::raster(&info, None, None).expect("surface");

        let typeface_font_provider = {
            let mut typeface_font_provider = TypefaceFontProvider::new();
            let font_mgr = FontMgr::new();
            let typeface = font_mgr
                .new_from_data(include_bytes!("../font/JetBrainsMono-Regular.ttf"), None)
                .expect("failed to load jb mono");

            typeface_font_provider.register_typeface(typeface, None);
            typeface_font_provider
        };

        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(Some(typeface_font_provider.into()), None);

        Self {
            size,
            buffer: vec![0u8; (size.width * size.height * 4) as usize],
            input: Input::default(),
            info,
            surface,
            font_collection,
            renderables: Vec::new(),
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

    fn fill_pixel_buffer(&mut self) {
        let rb = (self.size.width * 4) as usize;
        self.surface
            .read_pixels(&self.info, &mut self.buffer, rb, (0, 0));
    }

    // TODO: should probably not do this very often!
    pub fn create_byte_buffer<'local>(
        &mut self,
        env: &mut JNIEnv<'local>,
    ) -> jni::errors::Result<JByteBuffer<'local>> {
        unsafe { env.new_direct_byte_buffer(self.buffer.as_mut_ptr(), self.buffer.len()) }
    }

    fn end_draw(&mut self) {
        self.input.reset_scroll();
        self.input.reset_typed_characters();
        self.fill_pixel_buffer();
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn size(&self) -> &ISize {
        &self.size
    }

    pub fn on_mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        self.input.mouse_position = Position {
            x: mouse_x,
            y: mouse_y,
        }
    }

    pub fn on_key_pressed(&mut self, key_code: i32, scan_mode: i32, modifiers: i32) {
        self.input.key_state = Some(KeyState {
            key_code,
            scan_mode,
            modifiers,
        })
    }

    pub fn on_key_released(&mut self) {
        self.input.reset_key_state();
    }

    pub fn on_mouse_released(&mut self) {
        self.input.reset_mouse_button();
    }

    pub fn on_mouse_pressed(&mut self, button: i32) {
        self.input.mouse_button_down = match button {
            0 => Some(MouseButton::Left),
            1 => Some(MouseButton::Right),
            2 => Some(MouseButton::Middle),
            _ => None,
        };
    }

    pub fn on_mouse_scrolled(&mut self, delta_x: f64, delta_y: f64) {
        self.input.scroll_delta = Some(Delta {
            x: delta_x,
            y: delta_y,
        })
    }

    pub fn on_character_typed(&mut self, code_point: u16, modifiers: i32) {
        self.input.typed_characters.push_back(Character {
            code_point,
            modifiers
        })
    }

    // i really hate this but also why does Surface::canvas take a mutable ref
    // also it lets us wrap .end_draw nicely
    pub fn with_canvas(
        &mut self,
        f: impl FnOnce(&Canvas, &Input, &ISize, &FontCollection, &[Box<dyn ScreenRenderable>]),
    ) {
        // clear the canvas before we use it ...
        self.surface.canvas().clear(Color::from_argb(0, 0, 0, 0));

        f(
            self.surface.canvas(),
            &self.input,
            &self.size,
            &self.font_collection,
            &self.renderables,
        );

        self.end_draw();
    }

    pub fn render_all(&mut self) {
        self.surface.canvas().clear(Color::from_argb(0, 0, 0, 0));

        self.renderables.iter_mut().for_each(|f| {
            f.render(
                self.surface.canvas(),
                &self.input,
                &self.size,
                &self.font_collection,
            )
        });

        self.end_draw();
    }

    pub fn push_renderable(&mut self, renderable: Box<dyn ScreenRenderable>) {
        self.renderables.push(renderable);
    }

    pub fn is_renderables_empty(&self) -> bool {
        self.renderables.is_empty()
    }
}
