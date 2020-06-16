use crate::state::*;

use std::thread;


/// Fuck
/// ```
pub fn tprint(formated_str: &str) {
    println!("{:?}: {}", thread::current().id(), formated_str);
}

fn convert<'a>(flag : bool, t_str : &'a str, f_str : &'a str) -> &'a str {
    if flag {
        t_str
    }
    else {
        f_str
    }
}

/// Fuck
/// ```
pub fn format_state(state : usize) -> String {
    let t_str = "t";
    let f_str = "f";

    format!("[Hdr: Schedule:{}, Run:{}, Completed:{}, Closed:{}, Handle:{}, Awaiter:{}, Registering:{}, Notifying:{}, Rc:{}]",
                convert(state & SCHEDULED != 0, t_str, f_str),
                convert(state & RUNNING != 0, t_str, f_str),
                convert(state & COMPLETED != 0, t_str, f_str),
                convert(state & CLOSED != 0, t_str, f_str),
                convert(state & HANDLE != 0, t_str, f_str),
                convert(state & AWAITER != 0, t_str, f_str),
                convert(state & REGISTERING != 0, t_str, f_str),
                convert(state & NOTIFYING != 0, t_str, f_str),
                &(state / REFERENCE))
}