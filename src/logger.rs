use std::io::Write;
use env_logger::{Builder, fmt::Color};
use log::LevelFilter;

pub fn configure_logging(log_level: LevelFilter) {

    // intercept panics and log an error for them
    std::panic::set_hook(Box::new(|panic_info| {
        let msg = format!("{}", panic_info);
        log::error!("ERROR: {}", msg.replace("C:\\Users\\mjg99","").replace("mjg99","").replace("panicked at", ""));
    }));

    Builder::new()
        .format(|buf, record| {
            
            let mut path_style = buf.style();
            let mut style = buf.style();
            match record.level() {
                log::Level::Error => style.set_color(Color::Red).set_bold(true),
                log::Level::Warn => style.set_color(Color::Yellow).set_bold(true),
                log::Level::Info => style.set_color(Color::Green).set_bold(true),
                log::Level::Debug => style.set_color(Color::Rgb(128, 128, 128)).set_bold(false),
                log::Level::Trace => style.set_color(Color::Rgb(128, 128, 128)).set_bold(false),
            };

            path_style.set_color(Color::Rgb(128, 128, 128));
            let path = record.module_path().unwrap_or("<unknown>");
            
            writeln!(buf, "{} [{}] {} - {}",
                buf.timestamp(),
                style.value(record.level()),
                path_style.value(path),
                record.args())
        })
        // .target(env_logger::Target::Pipe(target))
        .target(env_logger::Target::Stdout)
        .filter(None, log_level)
        .init();
}