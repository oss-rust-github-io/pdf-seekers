//! # Logging module
//! 
//! - Defines the supporting function for setting Log4rs handle with associated configurations
//! - Provides flexibility to define log file path and log verbosity level

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::{
    roll::delete::DeleteRoller, trigger::size::SizeTrigger,
};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;


// Log file parameters
const ROLLING_SIZE_LIMIT: u64 = u64::pow(1024, 3);
const LOG_PATTERN_INFO: &str = "{h({d(%Y-%m-%d %H:%M:%S)(utc)} | {h({l}):5.5} | {m}{n})}";
const LOG_PATTERN_OTHERS: &str = "{h({d(%Y-%m-%d %H:%M:%S)(utc)} | {h({l}):5.5} | {f}:{L} - {m}{n})}";


/// Sets the log4rs handle with associated configurations
/// 
/// ## Input parameters:
/// - `log_file` defines the file path for creation of log files
/// - `log_level` defines the log verbosity level
/// 
/// ## Returns:
/// - Log4rs handle
pub fn set_logging(log_file: &String, log_level: &Option<String>) -> Result<log4rs::Handle, Box<dyn std::error::Error>> {
    // Define the log verbosity level based on input parameter
    let log_level: LevelFilter = match log_level {
        Some(s) => match s.to_string().to_lowercase().as_str() {
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "error" => LevelFilter::Error,
            "off" => LevelFilter::Off,
            _  => panic!("Invalid log verbosity level.")
        },
        None => LevelFilter::Info
    };

    // Create rolling trigger for log files
    let trigger: Box<SizeTrigger> = Box::new(SizeTrigger::new(ROLLING_SIZE_LIMIT));
    let roller: Box<DeleteRoller> = Box::new(DeleteRoller::new());
    let compound_policy: Box<CompoundPolicy> = Box::new(CompoundPolicy::new(trigger, roller));

    let file_logger_info: RollingFileAppender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN_INFO)))
        .build(log_file, compound_policy)?;

    let trigger: Box<SizeTrigger> = Box::new(SizeTrigger::new(ROLLING_SIZE_LIMIT));
    let roller: Box<DeleteRoller> = Box::new(DeleteRoller::new());
    let compound_policy: Box<CompoundPolicy> = Box::new(CompoundPolicy::new(trigger, roller));

    let file_logger_others: RollingFileAppender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN_OTHERS)))
        .build(log_file, compound_policy)?;
    
    // Create logger for writing to terminal
    let stdout_info: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN_INFO)))
        .build();

    let stdout_others: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_PATTERN_OTHERS)))
        .build();

    // Create logging config for Log4rs
    let config: Config = Config::builder()
        .appender(Appender::builder().build("stdout_info", Box::new(stdout_info)))
        .appender(Appender::builder().build("stdout_others", Box::new(stdout_others)))
        .appender(Appender::builder().build("file_logger_info", Box::new(file_logger_info)))
        .appender(Appender::builder().build("file_logger_others", Box::new(file_logger_others)))
        .logger(
            Logger::builder()
                .appender("stdout_info")
                .appender("file_logger_info")
                .build("info_logging", log_level),
        )
        .logger(
            Logger::builder()
                .appender("stdout_others")
                .appender("file_logger_others")
                .build("other_logging", log_level),
        )
        .build(
            Root::builder()
            .build(LevelFilter::Off)
        )?;

    // Create log4rs handle based on config defined
    let handle = log4rs::init_config(config)?;
    Ok(handle)
}