#[cfg(test)]
mod period_unit_test {
    use crate::domain_model::domain::models::period::Period;
    use chrono::NaiveDateTime;

    // INFO: Deve testar se período é válido;
    #[tokio::test]
    async fn test_if_period_is_valid() {
        let start_date =
            NaiveDateTime::parse_from_str("2024-05-29T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let end_date =
            NaiveDateTime::parse_from_str("2024-05-29T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let period = Period::new(start_date, end_date);
        assert!(period.is_ok());
    }

    // INFO: Deve testar a duração do período;
    #[tokio::test]
    async fn test_the_period() {
        let start_date =
            NaiveDateTime::parse_from_str("2024-05-29T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let end_date =
            NaiveDateTime::parse_from_str("2024-05-29T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let period = Period::new(start_date, end_date).expect("Failed to create period");

        assert_eq!(period.get_duration_in_hours(), 2.0);
    }

    // INFO: Deve testar se está fora do périodo;
    #[tokio::test]
    async fn test_if_period_is_in() {
        let start_date =
            NaiveDateTime::parse_from_str("2024-05-29T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let end_date =
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let period = Period::new(start_date, end_date).expect("Failed to create period");

        assert!(period.is_out_of_period(
            NaiveDateTime::parse_from_str("2024-05-29T07:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
        ));
    }
}
