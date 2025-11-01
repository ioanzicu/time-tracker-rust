#[derive(Debug, Default)]
pub struct HMSFormatter;
trait DurationFormat {
    fn format(&self, duration: std::time::Duration) -> String;
}

impl DurationFormat for HMSFormatter {
    fn format(&self, duration: std::time::Duration) -> String {
        let total_seconds = duration.as_secs();
        const HOUR: u64 = 3600; // sec
        const MINUTE: u64 = 60; // sec
        let hours = total_seconds / HOUR;
        let minutes = (total_seconds % HOUR) / MINUTE;
        let seconds = total_seconds % MINUTE;

        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn formats_seconds() {
        let duration = Duration::from_secs(5);

        let formatter = HMSFormatter::default();

        let text = formatter.format(duration);

        assert_eq!(&text, "00:00:05");
    }
}
