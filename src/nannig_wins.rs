use candelabre_core::CandlRenderer;
use candelabre_windowing::{
    CandlDimension, CandlOptions,
    CandlSurfaceBuilder
};
use gl;
use nvg_gl::Renderer;
use nvg::{Color, Context as NvgContext, Extent, Point, Rect};
use super::NannigState;

// NannigGraphics =============================================================

#[derive(Clone, Debug)]
pub enum NannigWinType {
    Classic,
    Clock,
    Config
}

pub struct NannigGraphics {
    context: Option<NvgContext<Renderer>>,
    size: (u32, u32),
    scale_factor: f32,
}

impl CandlRenderer<NannigGraphics, NannigState, ()> for NannigGraphics {
    fn init() -> NannigGraphics {
        NannigGraphics {
            context: None,
            size: (0, 0),
            scale_factor: 0.0
        }
    }

    fn finalize(&mut self) {
        let renderer = Renderer::create().unwrap();
        let mut context = NvgContext::create(renderer).unwrap();
        let font = include_bytes!("../resources/Berylium.ttf").to_vec();
        context.create_font("beryl", font).unwrap();
        self.context = Some(context);
    }

    fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor as f32;
    }

    fn set_size(&mut self, nsize: (u32, u32)) {
        self.size = nsize;
        let (w, h) = nsize;
        unsafe {
            gl::Viewport(
                0,
                0,
                (w as f32 *self.scale_factor) as i32,
                (h as f32 *self.scale_factor) as i32
            );
        }
    }

    fn draw_frame(&mut self, state: &NannigState) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT |
                gl::DEPTH_BUFFER_BIT |
                gl::STENCIL_BUFFER_BIT
            );
        }
        let (w, h) = self.size;
        if let Some(ctxt) = &mut self.context {
            ctxt.begin_frame(
                nvg::Extent {width: w as f32, height: h as f32},
                self.scale_factor
            ).unwrap();
            ctxt.save();
            //
            match state.win_type {
                NannigWinType::Config => NannigGraphics::draw_config(ctxt, state),
                _ => NannigGraphics::draw_clock(ctxt, state)
            }
            //
            ctxt.restore();
            ctxt.end_frame().unwrap();
        }
    }
}

impl NannigGraphics {
    fn draw_clock(ctxt: &mut NvgContext<Renderer>, _state: &NannigState) {
        //
        ctxt.fill_paint(Color::rgb_i(255, 0, 0));
        //
        ctxt.rect(Rect::new(Point::new(20.0, 20.0), Extent::new(50.0, 50.0)));
        //
        ctxt.fill().unwrap();
        //
    }

    fn draw_config(ctxt: &mut NvgContext<Renderer>, _state: &NannigState) {
        //
        ctxt.fill_paint(Color::rgb_i(0, 255, 0));
        //
        ctxt.rect(Rect::new(Point::new(10.0, 10.0), Extent::new(100.0, 20.0)));
        //
        ctxt.fill().unwrap();
        //
    }
}

// windows helpers ============================================================

fn gen_options() -> CandlOptions {
    CandlOptions::default()
        .set_vsync(true)
        .set_samples(4)
}

fn build_win(dim: CandlDimension, title: &str, win_type: NannigWinType)
-> CandlSurfaceBuilder<NannigGraphics, NannigState, ()> {
    CandlSurfaceBuilder::new()
        .dim(dim)
        .title(title)
        .options(gen_options())
        .render(NannigGraphics::init())
        .state(NannigState::new(win_type))
}

// windows creation functions =================================================

pub fn classic_win<'a>()
-> CandlSurfaceBuilder<'a, NannigGraphics, NannigState, ()> {
    build_win(
        CandlDimension::Classic(800, 600),
        "Nannig - Classic",
        NannigWinType::Classic
    )
}

pub fn fullscreen_win<'a>()
-> CandlSurfaceBuilder<'a, NannigGraphics, NannigState, ()> {
    build_win(
        CandlDimension::Fullscreen,
        "Nannig - FULLSCREEN",
        NannigWinType::Clock
    )
}

pub fn config_win<'a>()
-> CandlSurfaceBuilder<'a, NannigGraphics, NannigState, ()> {
    build_win(
        CandlDimension::Classic(400, 800),
        "Nannig - Configuration",
        NannigWinType::Config
    )
}
