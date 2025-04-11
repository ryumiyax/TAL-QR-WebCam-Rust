// Reference: https://github.com/esuo1198/TaikoArcadeLoader/blob/Refactor/PLUGINS.md

type CardCallback = extern "C" fn(i32, i32, *const u8, u64);
type CommitCardCallback = fn(String, String) -> bool;
type CommitQrCallback = fn(&mut Vec<u8>) -> bool;
type CommitQrLoginCallback = fn(String) -> bool;

#[unsafe(no_mangle)]
pub extern "stdcall" fn Init() {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn Update() {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn Exit() {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn InitVersion(version: u64) {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn InitCardReader(callback: CommitCardCallback) {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn InitQRScanner(callback: CommitQrCallback) {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn InitQRLogin(callback: CommitQrLoginCallback) {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn UpdateStatus(statusType: usize, status: bool) {
    
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn WaitTouch(callback: CardCallback, data: u64) {
    
}
