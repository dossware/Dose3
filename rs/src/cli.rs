use clap::{Command, Arg, ArgAction};

pub fn get_cli_args() -> clap::ArgMatches {
    Command::new("dose")
        .version("1.0")
        .about("🚀 Dose3 Starter Kit CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_version_flag(true)
        .arg(
            Arg::new("version")
                .short('v') 
                .long("version")
                .action(ArgAction::Version) 
                .help("Prints version information")
        )
        .subcommand(
            Command::new("init")
                .about("🛠️  Initialize a new project")
        )
        .subcommand(
            Command::new("check")
                .about("🔍 Check if Git and Node.js are installed"),
        )
        .subcommand(
            Command::new("ls")
                .about("📂 List files in the current directory"),
        )
        .subcommand(
            Command::new("ls2")
                .about("📁 List all files recursively with depth"),
        )
        .subcommand(
            Command::new("scan")
                .about("🔍 Scan open ports from 1 to 65535"),
        )
        .subcommand(
            Command::new("kill")
                .about("❌ Kill a specific port")
                .arg(
                    clap::Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("Port number to kill")
                        .required(true)
                        .value_parser(clap::value_parser!(u16)),
                ),
        )
        .subcommand(
            Command::new("sys")
                .about("🖥️  Show system status"),
        )
        .subcommand(
            Command::new("deps")
            .about("🔧 Check dependencies like Git and Node.js")
        )
        .subcommand(
            Command::new("fmt")
                .about("📝 Format files programmatically"),
        )
        .get_matches()
}
