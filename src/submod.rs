pub fn run() {
    for i in 0..10000 {
        log::error!("To test rolling file configurations we print this message in a loop. This is loop nr. {}", i);
    }
}