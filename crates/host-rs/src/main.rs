//use clap::Parser;
//use cli;
//use cli::cmd;

fn main() {
    //let c = cli::Cli::parse();
    //println!("{:?}", c.name);
    //
    /*
        let mm = cmd().get_matches();
        match mm.subcommand() {
            Some(("add", sync_matches)) => {
                if sync_matches.contains_id("block") {
                    let packages: Vec<_> = sync_matches
                        .get_many::<String>("block")
                        .expect("contains_id")
                        .map(|s| s.as_str())
                        .collect();
                    println!("Block Args: {:?}", packages);
                } else if sync_matches.contains_id("allow") {
                    let packages: Vec<_> = sync_matches
                        .get_many::<String>("allow")
                        .expect("contains_id")
                        .map(|s| s.as_str())
                        .collect();
                    println!("Allow Args: {:?}", packages);
                }
            }

            _ => unreachable!(),
        }

    */
}
