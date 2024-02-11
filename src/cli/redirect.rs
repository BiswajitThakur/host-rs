use std::collections::HashMap;

use host_rs::app::App;
pub fn init(app: &mut App, matches: &clap::ArgMatches, action: &mut bool) {
    let add_args = matches
            .get_many::<String>("add-redirect")
            .unwrap_or_default()
            .map(|v| v.clone())
            .collect::<Vec<_>>();
        if add_args.len() == 2 {
            *action = true;
            app.add_redirect_list(HashMap::from([(add_args[1].clone(), add_args[0].clone())]));
        };
        let rm_args = matches
            .get_many::<String>("rm-redirect")
            .unwrap_or_default()
            .map(|v| v.clone())
            .collect::<Vec<_>>();
       if rm_args.len() < 1 {
           return;
       };
       *action = true;
       app.rm_redirect(rm_args);
}