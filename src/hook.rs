use std::sync::Once;

use retour::static_detour;
use windows::{
    core::HRESULT,
    Win32::{
        Graphics::Gdi::{WindowFromDC, HDC},
        UI::WindowsAndMessaging::SetWindowLongPtrA,
    },
};

type FnWglSwapBuffers = unsafe extern "stdcall" fn(HDC) -> HRESULT;
static_detour! {
    static WglSwapBuffersHook: unsafe extern "stdcall" fn(HDC) -> HRESULT;
}

fn hook_wgl_swapbuffers(hdc: HDC) -> HRESULT {
    unsafe {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            println!("wglSwapBuffers successfully hooked.");

            let window = WindowFromDC(hdc);
            APP.init_default(hdc, window, ui);

            OLD_WND_PROC = Some(std::mem::transmute(SetWindowLongPtrA(
                window,
                GWLP_WNDPROC,
                hk_wnd_proc as usize as _,
            )));
        });

        APP.render(hdc);
        WglSwapBuffersHook.call(hdc)
    }
}
