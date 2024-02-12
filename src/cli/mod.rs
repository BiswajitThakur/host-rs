use host_rs::app::App;
mod ads;
mod fakenews;
mod gambling;
mod options;
mod porn;
mod redirect;
mod rm;
mod social;
mod update;
mod web;
pub struct Cli {}

impl Cli {
    pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let matches = options::init();
        let mut action = false;
        let mut app = App::init().unwrap();
        ads::init(&mut app, &matches, &mut action).await?;
        fakenews::init(&mut app, &matches, &mut action).await?;
        gambling::init(&mut app, &matches, &mut action).await?;
        porn::init(&mut app, &matches, &mut action).await?;
        rm::init(&mut app, &matches, &mut action)?;
        redirect::init(&mut app, &matches, &mut action);
        social::init(&mut app, &matches, &mut action).await?;
        update::init(&mut app, &matches, &mut action).await?;
        web::init(&mut app, &matches, &mut action);
        if action {
            app.flush()?;
        };
        Ok(())
    }
}
