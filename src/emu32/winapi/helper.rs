use lazy_static::lazy_static; 
use std::sync::Mutex;

lazy_static! {
    static ref HANDLERS:Mutex<Vec<u32>> = Mutex::new(vec![0;0]);
    static ref SOCKETS:Mutex<Vec<u32>> = Mutex::new(vec![0;0]);
}


pub fn handler_create() -> u32 {
    let new_handle:u32;
    let mut handles = HANDLERS.lock().unwrap();

    if handles.len() == 0 {
        new_handle = 1;
    } else {
        let last_handle = handles[handles.len()-1];
        new_handle = last_handle + 1;
    }
    
    handles.push(new_handle);
    return new_handle;
}

pub fn handler_close(hndl:u32) -> bool {
    let mut handles = HANDLERS.lock().unwrap();
    let idx = match handles.iter().position(|h| *h == hndl) {
        Some(i) => i,
        None => return false,
    };
    handles.remove(idx);
    return true;
}

pub fn handler_print() {
    let hndls = HANDLERS.lock().unwrap();
    for h in hndls.iter() {
        println!("{:x}", h);
    }
}

pub fn handler_exist(hndl:u32) -> bool {
    let handles = HANDLERS.lock().unwrap();
    match handles.iter().position(|h| *h == hndl) {
        Some(_) => return true,
        None => return false,
    }
}


pub fn socket_create() -> u32 {
    let new_socket:u32;
    let mut sockets = SOCKETS.lock().unwrap();

    if sockets.len() == 0 {
        sockets.push(0); // stdin
        sockets.push(1); // stdout
        sockets.push(2); // stderr
        new_socket = 3;  // first available socket
    } else {
        let last_socket = sockets[sockets.len()-1];
        new_socket = last_socket + 1;
    }
    
    sockets.push(new_socket);
    return new_socket;
}

pub fn socket_close(sock:u32) -> bool {
    let mut sockets = SOCKETS.lock().unwrap();
    let idx = match sockets.iter().position(|s| *s == sock) {
        Some(i) => i,
        None => return false,
    };
    sockets.remove(idx);
    return true;
}

pub fn socket_exist(sock:u32) -> bool {
    let sockets = SOCKETS.lock().unwrap();
    match sockets.iter().position(|s| *s == sock) {
        Some(_) => return true,
        None => return false,
    }
}

