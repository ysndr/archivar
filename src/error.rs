
use std;
use std::path::PathBuf;
use std::fmt;

use error_chain;

error_chain! {

    errors {
        InvalidCommandArgs(arg: String, content: String, fault: String){
            description("invalid input! could not make actions")
            display("invalid input! could not make actions
                    (argument: {}; content: {}, fault: {})",
                    arg, content, fault)
        }

    }



}
