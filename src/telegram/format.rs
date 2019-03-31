use failure::Error;

use crate::source::LogRecord;

pub fn error(error: Error) -> String {
    format!("Error: {}", error)
}

pub fn record(record: &LogRecord) -> String {
    format!("*{}*```\n{}```", record.title, record.body)
}

#[cfg(test)]
mod tests {
    use failure::err_msg;

    use crate::source::LogRecord;

    #[test]
    fn error_fmt() {
        let error = err_msg("oh no");
        let text = super::error(error);

        assert_eq!(text, "Error: oh no");
    }

    #[test]
    fn record_fmt() {
        let record = LogRecord {
            title: String::from("wow title"),
            body: String::from("such body"),
        };
        let text = super::record(&record);

        assert_eq!(text, "*wow title*```\nsuch body```");
    }
}
