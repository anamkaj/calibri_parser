use std::{ error::Error, thread::sleep, time::Duration as DuratTimer };
use chrono::NaiveDate;

//* Таймер*/
//*______________________________________________________________________ */
pub async fn timer(sec: u64) -> Result<(), Box<dyn Error>> {
    let duration = DuratTimer::from_secs(sec);
    let _ = sleep(duration);

    Ok(())
}

//* Трансформация даты для запросов через url*/
//*______________________________________________________________________ */
pub async fn date_transform(date: String) -> String {
    let transform_data: &str = "%d.%m.%Y";

    if date.trim().is_empty() {
        return String::new();
    } else {
        let start_date: NaiveDate = NaiveDate::parse_from_str(&date, transform_data).expect(
            "Ошибка трансформации"
        );
        start_date.to_string()
    }
}
