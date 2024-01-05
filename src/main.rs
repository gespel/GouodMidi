use std::thread;
use std::thread::Thread;
use std::time::Duration;
use crate::gmidiclient::GMidiClient;

mod gmidiclient;

fn main() {
    let mut gm = GMidiClient::new(0);
    loop {
        thread::sleep(Duration::from_secs(1));
        gm.play_note(60, 100);
        thread::sleep(Duration::from_secs(1));
        gm.stop_note(60, 100);
        //gm.list_all_outputs();
    }

}
