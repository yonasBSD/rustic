//! `tag` subcommand
use abscissa_core::{Command, Runnable};
use rustic_core::StringList;

use crate::commands::rewrite::RewriteCmd;

/// `tag` subcommand
#[derive(clap::Parser, Command, Debug)]
pub(crate) struct TagCmd {
    /// Snapshots to change tags. If none is given, use filter to filter from all
    /// snapshots.
    #[clap(value_name = "ID")]
    ids: Vec<String>,

    /// Tags to add (can be specified multiple times)
    #[clap(
        long,
        value_name = "TAG[,TAG,..]",
        conflicts_with = "remove",
        help_heading = "Tag options"
    )]
    add: Vec<StringList>,

    /// Tags to remove (can be specified multiple times)
    #[clap(long, value_name = "TAG[,TAG,..]", help_heading = "Tag options")]
    remove: Vec<StringList>,

    /// Tag list to set (can be specified multiple times)
    #[clap(
        long,
        value_name = "TAG[,TAG,..]",
        conflicts_with = "remove",
        help_heading = "Tag options"
    )]
    set: Vec<StringList>,
}

impl Runnable for TagCmd {
    fn run(&self) {
        let rewrite = RewriteCmd {
            ids: self.ids.clone(),
            add_tags: self.add.clone(),
            remove_tags: self.remove.clone(),
            set_tags: self.set.clone(),
            ..Default::default()
        };
        rewrite.run();
    }
}
