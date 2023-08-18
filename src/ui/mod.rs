use hudhook::hooks::{ImguiRenderLoop, ImguiRenderLoopFlags};
use imgui::*;

struct OuvertImgui;

impl ImguiRenderLoop for OuvertImgui {
    fn render(&mut self, ui: &mut Ui, _flags: &ImguiRenderLoopFlags) {
        ui.window("##hello")
            .size([320., 200.], Condition::Always)
            .build(|| {
                ui.text("Hello, world!");
            });
    }
}
