use candelabre_core::CandlRenderer;
use candelabre_windowing::{
    CandlDimension, CandlOptions,
    CandlSurfaceBuilder
};
use gl;
use nanovg::{self, Color};
use super::NannigState;

// NannigGraphics =============================================================

pub struct NannigGraphics {
    context: Option<nanovg::Context>,
    size: (u32, u32),
    scale_factor: f32
}

impl CandlRenderer<NannigGraphics> for NannigGraphics {
    fn init() -> NannigGraphics {
        NannigGraphics {
            context: None,
            size: (0, 0),
            scale_factor: 0.0
        }
    }

    fn finalize(&mut self) {
        let context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .build()
            .expect("Init of nanovg failed...");
        self.context = Some(context);
    }

    fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor as f32;
    }

    fn set_size(&mut self, nsize: (u32, u32)) { self.size = nsize; }

    fn draw_frame(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT |
                gl::DEPTH_BUFFER_BIT |
                gl::STENCIL_BUFFER_BIT
            );
        }
        let (w, h) = self.size;
        let size = (w as f32, h as f32);
        //
        println!("scale factor: {}", self.scale_factor);
        //
        self.context.as_ref().unwrap().frame(size, self.scale_factor, |frame| {
            //
            frame.path(|path| {
                path.rect((20.0, 20.0), (50.0, 50.0));
                //
                path.fill(Color::new(1.0, 0.0, 0.0, 1.0), Default::default());
                //
            }, Default::default());
            //
            //
        });
    }
}

impl NannigGraphics {
    //
    //
}

// windows helpers ============================================================

fn gen_options() -> CandlOptions {
    CandlOptions::default()
        .set_vsync(true)
        .set_samples(4)
}

fn build_win(dim: CandlDimension, title: &str)
-> CandlSurfaceBuilder<NannigGraphics, NannigState, ()> {
    CandlSurfaceBuilder::new()
        .dim(dim)
        .title(title)
        .options(gen_options())
        .render(NannigGraphics::init())
        .state(NannigState::new())
}

// windows creation functions =================================================

pub fn classic_win<'a>()
-> CandlSurfaceBuilder<'a, NannigGraphics, NannigState, ()> {
    build_win(CandlDimension::Classic(800, 400), "Nannig - Classic")
}

pub fn fullscreen_win<'a>()
-> CandlSurfaceBuilder<'a, NannigGraphics, NannigState, ()> {
    build_win(CandlDimension::Fullscreen, "Nannig - FULLSCREEN")
}

pub fn config_win<'a>()
-> CandlSurfaceBuilder<'a, NannigGraphics, NannigState, ()> {
    build_win(CandlDimension::Classic(400, 800), "Nannig - Configuration")
}