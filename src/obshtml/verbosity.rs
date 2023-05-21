// maximum level of logging to show
// (if you choose Info, Warning and Error will also be shown)
pub enum Verbosity {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}
pub struct MessageVerbosity(pub Verbosity);
pub struct ConfiguredVerbosity(pub Verbosity);

// compares whether the requested verbosity is higher or equal to the set verbosity
// which would mean that the message is allowed to be printed
pub fn verbose_enough(configured_verbosity: ConfiguredVerbosity, message_verbosity: MessageVerbosity) -> bool {
    let conf_level = configured_verbosity.0 as i32;
    let msg_level = message_verbosity.0 as i32;
    return msg_level >= conf_level;
}

/*  
    // Usage:
    let print = verbose_enough(ConfiguredVerbosity(Verbosity::Error), MessageVerbosity(Verbosity::Info));
    if print == true {
        println!("message printed");
    } else {
        println!("message hidden");
    }
*/