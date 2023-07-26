use poggers::{exports::HMODULE, internal::windows::module::InModule};

use lua::OuvertLua;

pub mod hook;
pub mod lua;
pub mod util;

struct OuvertState {
    module: InModule,
    lua: OuvertLua,
}

#[poggers_derive::create_entry(no_free)]
fn entry(hmodule: HMODULE) -> Result<(), Box<dyn std::error::Error>> {
    let module = InModule::new("openspades.exe")?;
    let lua = OuvertLua::new()?;

    let script = include_str!("../test.lua");

    match lua.load(script).exec() {
        Ok(_) => {
            println!("Exec");
        }
        Err(err) => return Err(Box::new(err)),
    }

    println!("Injected yayayay");

    Ok(())
}
