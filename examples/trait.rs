use clap::{App, Arg as ClapArg, ArgMatches, Error};

trait Arg<'help> {
    type Out;

    fn new() -> ClapArg<'help>;

    fn parse_get(matches: &ArgMatches) -> Option<Self::Out>;
}

trait Command<'help> {
    fn new() -> App<'help>;

    fn run(matches: &ArgMatches) -> Result<(), Error>;
}

#[derive(Debug)]
struct NameArg {}

impl<'help> Arg<'help> for NameArg {
    type Out = String;

    fn new() -> ClapArg<'help> {
        ClapArg::new("name")
            .long("name")
            .about("name to say hello")
            .value_name("name")
    }

    fn parse_get(matches: &ArgMatches) -> Option<Self::Out> {
        Some(matches.value_of("name").unwrap().to_string())
    }
}

#[derive(Debug)]
struct HelloCommand {}

impl<'help> Command<'help> for HelloCommand {
    fn new() -> App<'help> {
        App::new("hello").arg(NameArg::new())
    }

    fn run(matches: &ArgMatches) -> Result<(), Error> {
        let name = NameArg::parse_get(&matches);

        println!("Hello, {:?}", name);

        Ok(())
    }
}

fn main() {
    let matches = HelloCommand::new().get_matches();
    HelloCommand::run(&matches);
}
