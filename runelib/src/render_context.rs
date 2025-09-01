use std::rc::Rc;
use jni::sys::jlong;
use skia_safe::{AlphaType, Canvas, Color, ColorType, ISize, ImageInfo, Paint, PaintStyle, Surface, surfaces, FontMgr, FontStyle, Font, SurfaceProps, SurfacePropsFlags, PixelGeometry};
use skia_safe::textlayout::{FontCollection, FontFeature, ParagraphBuilder, ParagraphStyle, TextStyle, TypefaceFontProvider};

#[derive(Default, Debug)]
pub struct KeyState {
    key_code: i32,
    scan_mode: i32,
    modifiers: i32,
}

pub struct RenderContext {
    pub size: ISize,
    pub buffer: Vec<u8>,
    info: ImageInfo,
    pub surface: Surface,
    pub key_state: Option<KeyState>,
    pub mouse_button: Option<i32>,
    pub mouse_scroll: Option<(f64, f64)>,

    // i hate fonts !
    pub font_collection: FontCollection,
    pub mono_ts: TextStyle,
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
        let props = SurfaceProps::new(SurfacePropsFlags::empty(), PixelGeometry::BGRH);
        let surface = surfaces::raster(&info, None, Some(&props)).expect("surface");

        let typeface_font_provider = {
            let mut typeface_font_provider = TypefaceFontProvider::new();
            let font_mgr = FontMgr::new();
            let typeface = font_mgr
                .new_from_data(include_bytes!("font/JetBrainsMono-Regular.ttf"), None)
                .expect("failed to load jb mono");

            typeface_font_provider.register_typeface(typeface, None);
            typeface_font_provider
        };

        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(Some(typeface_font_provider.into()), None);

        let mut regular_ts = TextStyle::new();
        // regular_ts.set_font_size(16.0);

        let mut mono_ts = TextStyle::new();
        // mono_ts.set_font_size(13.0);
        // mono_ts.set_typeface(tf);

        // https://learn.microsoft.com/en-us/typography/opentype/spec/featurelist
        // i dont really know what all i need to enable ;sob
        // fk it enable all ligatures
        // mono_ts.add_font_feature("liga", 1); // standard ligatures
        // mono_ts.add_font_feature("clig", 1); // contextual
        // mono_ts.add_font_feature("dlig", 1); // discretionary
        // mono_ts.add_font_feature("hlig", 1); // historical
        // mono_ts.add_font_feature("rlig", 1); // required

        Self {
            size,
            info,
            surface,
            buffer: vec![0u8; (size.width * size.height * 4) as usize],
            key_state: None,
            mouse_button: None,
            mouse_scroll: None,
            font_collection,
            mono_ts
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

    fn reset_inputs(&mut self) {
        self.mouse_button = None;
        self.mouse_scroll = None;
        self.key_state = None;
    }

    pub fn draw<F>(&mut self, callback: F)
    where
        F: Fn(&mut Self),
    {
        callback(self);

        self.reset_inputs();
        self.fill_pixel_buffer();
    }

    pub fn update_key_state(&mut self, key_code: i32, scan_mode: i32, modifiers: i32) {
        self.key_state = Some(KeyState {
            key_code,
            scan_mode,
            modifiers,
        })
    }

    pub fn update_mouse_press_state(&mut self, button: i32) {
        self.mouse_button = Some(button)
    }

    pub fn update_mouse_scroll_state(&mut self, delta_x: f64, delta_y: f64) {
        self.mouse_scroll = Some((delta_x, delta_y))
    }
}
