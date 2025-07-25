
pub fn set_logger_format() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"); // 你想要的格式
            writeln!(buf, "[{} {} {}] {}",
                     ts,
                     record.level(),
                     record.target(),
                     record.args()
            )
        })
        .init();
}