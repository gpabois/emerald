use mlua::{Function, Lua, Result, Table, UserData, Value};

use crate::{
    fs::{self, DirEntry, File, Metadata},
    path::Path,
};

use super::Context;

fn fs_walk(lua: &Lua, path: String) -> Result<Function<'_>> {
    let path = Path::new(&path).unwrap();
    let ctx = lua.app_data_ref::<Context>().unwrap();
    let mut walk = fs::walk(&ctx.emerald, &path).unwrap();
    lua.create_function_mut(move |lua, ()| {
        let next = walk.next();
        Ok(match next {
            Some(entry) => lua.pack(lua.create_ser_userdata(entry)?)?,
            None => Value::Nil,
        })
    })
}

fn fs_open(lua: &Lua, (path, _mode): (String, u8)) -> Result<Value<'_>> {
    let ctx = lua.app_data_ref::<Context>().unwrap();
    let path = Path::new(&path).unwrap();
    let file = fs::open(&ctx.emerald, &path).unwrap();
    let val = lua.create_userdata(file)?;

    Ok(Value::UserData(val))
}

pub fn create_fs_table(lua: &Lua) -> Result<Table<'_>> {
    let fs = lua.create_table()?;

    fs.set("READ", 2)?;
    fs.set("WRITE", 4)?;

    fs.set("walk", lua.create_function(fs_walk)?)?;
    fs.set("open", lua.create_function(fs_open)?)?;

    Ok(fs)
}

impl UserData for File {}

impl UserData for Path {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("tostring", |_, this, ()| Ok(this.to_string()));
    }
}

impl UserData for Metadata {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("is_shard", |_, this| Ok(this.is_shard()));
        fields.add_field_method_get("is_file", |_, this| Ok(this.is_file()));
        fields.add_field_method_get("is_dir", |_, this| Ok(this.is_dir()));
    }
}

impl UserData for DirEntry {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("path", |_, this| Ok(this.path().clone()));
        fields.add_field_method_get("metadata", |_, this| Ok(this.metadata().clone()));
    }
}
