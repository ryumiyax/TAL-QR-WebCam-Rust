// Reference: https://github.com/esuo1198/TaikoArcadeLoader/blob/Refactor/PLUGINS.md

type CardCallback = extern "C" fn(i32, i32, *const u8, u64);

#[unsafe(no_mangle)]
pub extern "stdcall" fn Init() {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn Update() {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn WaitTouch(callback: CardCallback, data: u64) {
    
}
