use mlua::Lua;
use std::ops::Deref;

pub mod misc;

pub struct OuvertLua {
    pub lua: Lua,
}

impl Deref for OuvertLua {
    type Target = Lua;

    fn deref(&self) -> &Self::Target {
        &self.lua
    }
}

impl OuvertLua {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let lua = Lua::new();

        Self::init(&lua)?;

        Ok(Self { lua: lua })
    }

    fn init(lua: &Lua) -> Result<(), Box<dyn std::error::Error>> {
        let globals = lua.globals();

        globals.set("print", lua.create_function(misc::print)?)?;

        let memory = lua.create_table()?; // TODO: Create macro for memory table

        Ok(())
    }
}
