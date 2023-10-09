pub trait CliCommand {
    fn run(args: &Vec<String>) -> ();
}
