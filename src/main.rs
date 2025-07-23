extern crate clap;
extern crate dot;

use clap::{AppSettings, Arg, SubCommand};
use dot::App;

pub fn main() {
    match run() {
        Ok(retcode) => std::process::exit(retcode),
        Err(err) => panic!("unknown error: {}", err),
    }
}

pub fn run() -> dot::Result<i32> {
    let matches = cli().get_matches();
    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");

    let mut app = App::new(dry_run, verbose)?;

    match matches.subcommand() {
        Some(("check", _)) => app.command_check(),
        Some(("link", _)) => app.command_link(),
        Some(("clean", _)) => app.command_clean(),
        Some(("root", _)) => app.command_root(),

        Some(("clone", args)) => {
            let url = args.value_of("url").unwrap();
            app.command_clone(url)
        }

        Some(("init", args)) => {
            let url = args.value_of("url").unwrap();
            let ret = app.command_clone(url)?;
            if ret != 0 {
                return Ok(ret);
            }
            app.command_link()
        }

        Some(("completion", args)) => {
            let shell: clap_complete::Shell = args.value_of_t_or_exit("shell");
            clap_complete::generate(
                shell,
                &mut cli(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
            Ok(0)
        }

        Some(..) | None => unreachable!(),
    }
}

fn cli() -> clap::Command<'static> {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .help("Use verbose output")
                .long("verbose")
                .short('v'),
        )
        .arg(
            Arg::with_name("dry-run")
                .help("do not actually perform I/O operations")
                .long("dry-run")
                .short('n'),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("Check the files are correctly linked to the right places"),
        )
        .subcommand(
            SubCommand::with_name("link")
                .about("Create all of the symbolic links into home directory"),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Remote all of registered links from home directory"),
        )
        .subcommand(
            SubCommand::with_name("root")
                .about("Show the location of dotfiles repository and exit"),
        )
        .subcommand(
            SubCommand::with_name("clone")
                .about("Clone dotfiles repository from remote")
                .arg(
                    Arg::with_name("url")
                        .help("URL of remote repository")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Clone dotfiles repository from remote & make links")
                .arg(
                    Arg::with_name("url")
                        .help("URL of remote repository")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("completion")
                .about("Generate completion scripts")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("shell")
                        .help("target shell")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh", "powershell"]),
                ),
        )
}
