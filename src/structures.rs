use structopt::StructOpt;
use serde::{Deserialize, Serialize};

#[derive(StructOpt, Debug, Clone)]
pub struct UserCommand {
    pub command: String,

    #[structopt(long, short)]
    pub description: Option<String>,

    #[structopt(long, short)]
    pub login: Option<String>,

    #[structopt(required_if("command", "new"), required_if("command", "get"),
    required_if("command", "change"), required_if("command", "delete"))]
    pub resource: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Password {
    pub resource: String,
    pub login: String,
    pub password: String,
    pub description: String,
}