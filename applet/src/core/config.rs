pub fn init() {
    init_logger();
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "cosmic_ext_tweaks_applet=info");
    pretty_env_logger::init();
}
