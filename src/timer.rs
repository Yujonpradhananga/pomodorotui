use ratatui::DefaultTerminal;
use std::thread::sleep;
use std::time::Duration;
//logic
pub fn timer(terminal: &mut DefaultTerminal) {
    sleep(Duration::new(0, 0));
    let mut i: i32 = 0;
    let time: i32 = 1500;
    while i <= time {
        println!("{i}");
        sleep(Duration::new(1, 0));
        let var_name = terminal.clear();
        match var_name {
            Ok(_var_name) => {}
            Err(_) => todo!(),
        }
        i += 1;
    }
}
