use retour_utils::hook_module;

#[hook_module("OpenSpades.exe")]
#[allow(non_snake_case)]
pub mod OpenSpades {
    type This = *mut ();

    #[hook(unsafe extern "thiscall" NetClient_DoEvents, offset = 0x3878)]
    pub fn netclient_doevents(this: This, dt: i32) {
        println!("Hey it got called");

        unsafe {
            NetClient_DoEvents.call(this, dt);
        }
    }
}

#[hook_module("opengl32.dll")]
#[allow(non_snake_case)]
pub mod OpenGL {
    use windows::core::HRESULT;
    use windows::Win32::Graphics::Gdi::HDC;

    #[hook(unsafe extern "stdcall" owglSwapBuffers, symbol = "wglSwapBuffers")]
    #[allow(unused)]
    pub fn wglSwapBuffers(hdc: HDC) -> HRESULT {
        unsafe { owglSwapBuffers.call(hdc) }
    }
}
