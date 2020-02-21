use candelabre_core::CandlRenderer;
use gl;
use nanovg;

pub struct NannigGraphics {
    context: Option<nanovg::Context>
    //
}

impl CandlRenderer<NannigGraphics> for NannigGraphics {
    fn init() -> NannigGraphics {
        //
        /*
        let context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .build()
            .expect("Init of nanovg failed...");
        */
        //
        //NannigGraphics { context }
        NannigGraphics { context: None }
    }

    fn finalize(&mut self) {
        //
        let context = nanovg::ContextBuilder::new()
            .stencil_strokes()
            .build()
            .expect("Init of nanovg failed...");
        //
        self.context = Some(context);
        //
    }

    fn draw_frame(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            //
            gl::Clear(gl::COLOR_BUFFER_BIT);
            //
        }
        //
        //
    }
}
