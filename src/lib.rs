#![cfg(windows)]

use std::thread::{self, JoinHandle};
use std::fs;

use mlua::{Lua};

use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi;

use jni::JNIEnv;
use jni::objects::{JClass, JValue};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn init()
{
    unsafe { consoleapi::AllocConsole() };
    println!("Testing console..");
    println!("Console is good, exiting DLLMain");
}

static mut JNI_ENV: Option<JNIEnv> = Option::None;

// Function for Lua to call to draw a pixel on the screen
unsafe fn draw_pix(x: u32, y: u32)
{
    let dllhandler: JClass;
    match JNI_ENV.expect("JNI_ENV is None").find_class("bluefire_06/mimosa_computers/lua/DLLHandler") {
        Ok(class) => dllhandler = class,
        Err(e) => {println!("{}", e.to_string()); dllhandler = JClass::default()}
    }

    let args: [JValue; 2] = [JValue::Int(x.try_into().expect("X value is out of range")), JValue::Int(y.try_into().expect("Y value is out of range"))];

    match JNI_ENV.expect("JNI_ENV is None").call_static_method(dllhandler, "drawPix", "(II)V", &args) {
        Ok(_) => (),
        Err(e) => println!("{}", e.to_string())
    }
}

fn register_lua_functions(lua: Lua) -> Lua
{
    let setpixel = lua.create_function(|_, (x, y): (u32, u32)| {
        unsafe { draw_pix(x, y); }
        Ok(())
    }).expect("Error detected wile executing draw_pix");

    let gpu_table = lua.create_table().expect("");

    gpu_table.set("set", setpixel).expect("msg");

    lua.globals().set("gpu", gpu_table).expect("Error detected while trying to create GPU table");

    lua
}

#[no_mangle]
pub extern "system" fn Java_bluefire_106_mimosa_1computers_lua_DLLHandler_initLuaDLL(env: JNIEnv, _class: JClass)
{
    // We need JNI_ENV in registered Lua functions, so JNI_ENV needs to be a global.
    // And no, it isn't possible to get Lua to pass in the JNI enviorment for us, as
    // that would require Lua to be able to access the JNIEnv, which is a *massive*
    // security risk.
    
    let envCopy = env.clone();
    unsafe {
        JNI_ENV = Some(envCopy);
    }

    let dllhandler: JClass;
    match env.find_class("bluefire_06/mimosa_computers/lua/DLLHandler") {
        Ok(class) => dllhandler = class,
        Err(e) => {println!("{}", e.to_string()); dllhandler = JClass::default()}
    }


    let args: [JValue; 2] = [JValue::Int(1), JValue::Int(2)];

    match env.call_static_method(dllhandler, "drawPix", "(II)V", &args) {
        Ok(_) => println!("a"),
        Err(e) => println!("{}", e.to_string())
    }
}

#[no_mangle]
pub extern "system" fn Java_bluefire_106_mimosa_1computers_lua_DLLHandler_createComputerEnvDLL(_env: JNIEnv, _class: JClass)
{
    // Load BIOS before creating new thread, as std::fs doesn't like loading files in threads
    let bios = fs::read_to_string("./bios.lua").expect("Failed to load BIOS file! Please reinstall MimosaComputers!");

    let _t: JoinHandle<()> = thread::spawn(move|| {
        let mut lua: Lua = Lua::new();
        lua = register_lua_functions(lua);
        
        // Load BIOS into Lua
        lua.load(&bios).exec().expect("Failed to load BIOS, this is a catastrophic error! Please open a GitHub issue!");

        // Keep thread running
        loop {
            
        }
    });
}