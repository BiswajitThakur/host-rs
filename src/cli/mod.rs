use host_rs::app::App;
mod options;
mod web;
pub struct Cli {}

#[allow(dead_code)]
impl Cli {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        let matches = options::init();
        let mut action = false;
        let mut app = App::init().unwrap();
        web::init(&mut app, &matches, &mut action);
        if action {
            dbg!(&app);
            app.flush()?;
        };
        Ok(())
    }
}
