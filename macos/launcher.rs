use std::os::unix::process::ExitStatusExt;

fn main() {
    let exe = std::env::current_exe().unwrap();
    let game = exe.file_name().unwrap().to_str().unwrap();
    let macos_dir = exe.parent().unwrap();
    let resources_dir = macos_dir.parent().unwrap().join("Resources");
    let lua_lib = resources_dir.join("lua/lib");
    let lua_share = resources_dir.join("lua/share");

    if lua_lib.exists() {
        let base = format!("{0}/?.so;{0}/?/init.so", lua_lib.display());
        let existing = std::env::var("LUA_CPATH").unwrap_or_default();
        let val = if existing.is_empty() { base } else { format!("{base};{existing}") };
        unsafe { std::env::set_var("LUA_CPATH", val); }
    }

    if lua_share.exists() {
        let base = format!("{0}/?.lua;{0}/?/init.lua", lua_share.display());
        let existing = std::env::var("LUA_PATH").unwrap_or_default();
        let val = if existing.is_empty() { base } else { format!("{base};{existing}") };
        unsafe { std::env::set_var("LUA_PATH", val); }
    }

    let status = std::process::Command::new(macos_dir.join("rusty-path-of-building"))
        .arg(game)
        .args(std::env::args().skip(1))
        .status()
        .expect("Failed to launch rusty-path-of-building");
    let code = status.code().unwrap_or_else(|| status.signal().map_or(1, |s| 128 + s));
    std::process::exit(code);
}
