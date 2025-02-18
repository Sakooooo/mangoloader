use mlua::Lua;
use std::fs;

pub fn test_lua() {
    let lua = Lua::new();
    let script = fs::read_to_string("test.lua").expect("blyat");
    lua.load(&script).exec();
}
