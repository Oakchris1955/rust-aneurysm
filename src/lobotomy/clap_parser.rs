// we could probably use a function with generics instead, but that also works
/// [`shellfish`]'s `clap_command!` macro doesn't seem to work, because it uses an outdated version of [`Clap`](clap), so we gotta make our own
#[macro_export]
macro_rules! clap_command {
    ($state: ty, $args: ty, $fn: expr) => {{
        fn wrapper(
            state: &mut $state,
            args: Vec<String>,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let parsed_args: $args = match Parser::try_parse_from(&args) {
                Ok(o) => o,
                Err(err) => {
                    err.print()?;
                    return Ok(());
                }
            };
            $fn(state, parsed_args)?;
            Ok(())
        }

        let command = shellfish::Command::new(
            <$args as clap::CommandFactory>::command()
                .get_about()
                .unwrap()
                .to_string(),
            wrapper,
        );

        command
    }};
}
