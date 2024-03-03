use std::error::Error;

mod fs;

use mlua::Lua;

use crate::Emerald;

#[derive(Default)]
pub struct ScriptEngine {}

impl ScriptEngine {
    /// Creates a new script engine
    pub fn new() -> Self {
        Self {}
    }
    /// Create a new execution context.
    pub fn new_instance(&mut self, emerald: &Emerald) -> Result<Instance, Box<dyn Error>> {
        let lua = Lua::new();
        Context::inject(&lua, emerald)?;
        Ok(Instance { lua })
    }
}

/// Instance context
pub struct Context {
    pub emerald: Emerald,
}

impl Context {
    fn new(emerald: &Emerald) -> Self {
        Self {
            emerald: emerald.clone(),
        }
    }
    pub fn inject(lua: &Lua, emerald: &Emerald) -> Result<(), Box<dyn Error>> {
        lua.set_app_data(Self::new(emerald));
        Self::bind(lua)?;
        Ok(())
    }

    fn bind(lua: &Lua) -> Result<(), Box<dyn Error>> {
        let api = lua.create_table()?;
        let fs = fs::create_fs_table(lua)?;

        api.set("fs", fs)?;

        lua.globals().set("emerald", api)?;

        Ok(())
    }
}

pub struct Instance {
    lua: Lua,
}

impl Instance {
    pub fn execute(&self, code: &str) -> Result<(), Box<dyn Error>> {
        self.lua.load(code).exec()?;
        Ok(())
    }
}
