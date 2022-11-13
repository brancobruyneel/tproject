use clap::Command;

pub(crate) fn create_app() -> Command {
    Command::new("tp")
        .author("Branco Bruyneel <branco.bruyneel@gmail.com>")
        .version(clap::crate_version!())
        .about("Fuzzy search your way through your tmux sessions")
        .subcommand(
            Command::new("add")
                .arg_required_else_help(true)
                .about("Configure the defaults for search paths and excluded directories"),
        )
}
