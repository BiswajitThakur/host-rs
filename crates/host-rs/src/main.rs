use cli::CliApp;

fn main() {
    let app = CliApp::init(
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );
    app.run(env!("CARGO_BIN_NAME"));
}
