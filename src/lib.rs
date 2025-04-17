#![no_std]
#![allow(warnings)]
use gstd::*;
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

pub fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}



#[no_mangle]
pub extern "C" fn init() {
    // Receives `PebblesInit` using the `msg::load` function
    let load_init_msg = msg::load::<PebblesInit>().expect("Unable to load message");


#[no_mangle]
pub extern "C" fn handle() {

}

#[no_mangle]
pub extern "C" fn state() {
    
}


