use poggers::{exports::HMODULE, structures::process::{Process, implement::utils::ProcessUtils}};

use lua::OuvertLua;

pub mod hooks;
pub mod lua;
pub mod util;

// struct OuvertState {
//     module: InModule,
//     lua: OuvertLua,
// }

#[poggers_derive::create_entry(no_free)]
fn entry(_hmodule: HMODULE) -> Result<(), Box<dyn std::error::Error>> {
    let process = Process::this_process();
    let module  = process.get_module("OpenSpades.exe")?;
    let base_address = module.get_base_address();

    println!("Injected yayayay: {:x}", base_address);

    let lua = OuvertLua::new()?;

    let script = include_str!("../test.lua");

    match lua.load(script).exec() {
        Ok(_) => {
            println!("Exec");
        }
        Err(err) => return Err(Box::new(err)),
    }


    Ok(())
}
