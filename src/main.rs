mod submod;

fn main() {
    log4rs::init_file("logging.yaml", Default::default()).unwrap();
    
    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");

    submod::run();

    
}