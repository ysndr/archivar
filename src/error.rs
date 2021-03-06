error_chain! {

    errors {
         InvalidCommandArgs(arg: String, content: String, fault: String){
            description("invalid input")
            display("invalid input, could not make actions (argument: {}; content: {}, fault: {})",
                    arg, content, fault)
        }
         CommandUnknown(command: String){
            description("invalid command")
            display("invalid command ({}) issued", command)
        }
        Template


    }

    foreign_links {
        Shell(::shell::errors::Error);
        Io(::std::io::Error);
        FsExtra(::fs_extra::error::Error);
        SerdeYaml(::serde_yaml::Error);
    }

}
