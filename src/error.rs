use clap;
use std;
use std::fmt;
use std::path::PathBuf;

use error_chain;

error_chain! {

    errors {
         InvalidCommandArgs(arg: String, content: String, fault: String){
            description("invalid input! could not make actions")
            display("invalid input! could not make actions\n  (argument: {}; content: {}, fault: {})",
                    arg, content, fault)
        }
         CommandUnknown(command: String){
            description("invalid command")
            display("invalid command ({}) issued", command)
        }

    }

    foreign_links {
        Clap(::clap::Error);
    }

}
