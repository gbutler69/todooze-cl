use std::path::PathBuf;
use structopt::clap::ArgGroup;
use structopt::StructOpt;

/// Command-Line driven TODO application
///
/// # Example
/// Show Help about the commands by invocation of the 'help' command from the command-line...
/// ```sh
/// $> todooze-cl help
/// ```
/// Parse arguments (in Main for example):
/// ```
/// use todooze_cl::opt::Opt;
/// use structopt::StructOpt;
///
/// let opt = Opt::from_args_safe();
/// # let opt = Opt::from_iter_safe(&["help"]);
/// match opt {
///    Ok(opt) => assert!(false,"'help' should NOT populate the options struct"),
///    Err(e) => {
///         assert!(e.message.starts_with("todooze-cl"),"'help' should cause an error containing the help text");
///         assert!(e.message.ends_with("USAGE:\n    help <SUBCOMMAND>\n\nFLAGS:\n    -h, --help       Prints help information\n    -V, --version    Prints version information\n\nSUBCOMMANDS:\n    add       Add an item\n    delete    Delete an Item\n    help      Prints this message or the help of the given subcommand(s)\n    list      List Summary of Items of Categories\n    report    Report on Items or Categories\n    run       Run multiple commands from stdin or a file/file-handle\"\n    show      Show an Item\n    update    Update an Item"))
///     }
/// }
/// ```
#[derive(Debug, StructOpt)]
#[structopt(name = "todooze-cl")]
pub enum Opt {
    /// Run multiple commands from stdin or a file/file-handle"
    Run(Run),
    /// Add an item
    Add(Add),
    /// Delete an Item
    Delete(Delete),
    /// Update an Item
    Update(Update),
    /// Show an Item
    Show(Show),
    /// List Summary of Items of Categories
    List(List),
    /// Report on Items or Categories
    Report(Report),
}

/// Additional Congfiguration Options
#[derive(Debug, StructOpt)]
#[structopt(group=ArgGroup::with_name("output").required(false).multiple(false),
            group=ArgGroup::with_name("errors").required(false).multiple(false))]
pub struct Config {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Output file, stdout if not present & output-handle (mutually exclusive with this option) is not present
    #[structopt(long, parse(from_os_str), group = "output")]
    output_file: Option<PathBuf>,

    /// Output handle (signed-integer), stdout if not present & output-file (mutually exclusive with this option) is not present
    #[structopt(long, group = "output")]
    output_handle: Option<i64>,

    /// Error file, stderr if not present & errors-handle (mutually exclusive with this option) is not present
    #[structopt(long, parse(from_os_str), group = "errors")]
    errors_file: Option<PathBuf>,

    /// Error handle (signed-integer), stderr if not present & errors-file (mutually exclusive with this option) is not present
    #[structopt(long, group = "errors")]
    errors_handle: Option<i64>,
}

/// File Specification
#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the Run command
pub struct Run {
    #[structopt(flatten)]
    config: Config,

    /// Input file for commands, stdin if not present
    #[structopt(parse(from_os_str))]
    commands: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
/// Categories of items
pub enum Category {
    Resource(Resource),
    ToDo(ToDo),
    Project(Project),
    State(State),
    Action(Action),
}

#[derive(Debug, StructOpt)]
/// Fields for a Resource
pub struct Resource {}

#[derive(Debug, StructOpt)]
/// Fields for a ToDo
pub struct ToDo {}

#[derive(Debug, StructOpt)]
/// Fields for a Project
pub struct Project {}

#[derive(Debug, StructOpt)]
/// Fields for a State
pub struct State {}

#[derive(Debug, StructOpt)]
/// Fields for a Action
pub struct Action {}

#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the Add command
pub struct Add {
    #[structopt(flatten)]
    config: Config,

    #[structopt(subcommand)]
    category: Category,
}

#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the Delete command
pub struct Delete {
    #[structopt(flatten)]
    config: Config,

    #[structopt(subcommand)]
    category: Category,
}

#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the Update command
pub struct Update {
    #[structopt(flatten)]
    config: Config,

    #[structopt(subcommand)]
    category: Category,
}

#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the Show command
pub struct Show {
    #[structopt(flatten)]
    config: Config,

    #[structopt(subcommand)]
    category: Category,
}

#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the List command
pub struct List {
    #[structopt(flatten)]
    config: Config,

    #[structopt(subcommand)]
    category: Category,
}

#[derive(Debug, StructOpt)]
/// Options & Sub-Commands for the Report command
pub struct Report {
    #[structopt(flatten)]
    config: Config,

    #[structopt(subcommand)]
    category: Category,
}
