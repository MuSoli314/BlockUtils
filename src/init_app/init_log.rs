use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Config;

pub fn init_log(level: &str, file_path: &str, is_stderr: bool, roller_num: u32) {
    let level = get_level(level);

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let roller_file_name = format!("{}{{}}", file_path);
    let fixed_window_roller = FixedWindowRoller::builder()
        .build(roller_file_name.as_str(), roller_num)
        .unwrap();

    let roller_file_size_limit = 1024 * 1024 * 100; // Default: 100MB
    let roller_file_trigger = SizeTrigger::new(roller_file_size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(roller_file_trigger), Box::new(fixed_window_roller));

    let config = Config::builder().appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(level)))
            .build(
                "logfile",
                Box::new(
                    RollingFileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(
                            "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}",
                        )))
                        .build(file_path, Box::new(compound_policy))
                        .unwrap(),
                ),
            ),
    );

    if is_stderr {
        let c = config
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(level),
            )
            .unwrap();
        let _ = log4rs::init_config(c).unwrap();
    } else {
        let c = config
            .build(Root::builder().appender("logfile").build(level))
            .unwrap();
        let _ = log4rs::init_config(c).unwrap();
    }
}

fn get_level(level: &str) -> LevelFilter {
    let _level = level.to_lowercase();
    let mut level = LevelFilter::Off;
    if _level.eq("trace") {
        level = LevelFilter::Trace
    } else if _level.eq("debug") {
        level = LevelFilter::Debug
    } else if _level.eq("info") {
        level = LevelFilter::Info
    } else if _level.eq("warn") {
        level = LevelFilter::Warn
    } else if _level.eq("error") {
        level = LevelFilter::Error
    }

    return level;
}

#[cfg(test)]
mod tests {
    use log::{debug, error, info, warn};

    use crate::init_app::init_log::init_log;

    #[test]
    fn it_works() {
        init_log("debug", "Logs/test.log", true, 4);
        debug!("AAAAA");
        info!("BBBBB");
        error!("CCCC");
        warn!("DDDDDD");
    }
}
