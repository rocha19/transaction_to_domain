#[cfg(test)]
mod parking_domain_model {
    use chrono::NaiveDateTime;
    use std::sync::Arc;
    use value_object::domain_model::{
        application::usecases::{checkin::CheckinUseCase, checkout::CheckoutUseCase},
        domain::models::period::Period,
        infra::in_memory_parked_car_repository::InMemoryParkedCarRepository,
    };

    // INFO: Teste de check-in e check-out simples
    #[tokio::test]
    async fn test_checkin_checkout() {
        let working_hours = Period::new(
            NaiveDateTime::parse_from_str("2024-05-29T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )
        .unwrap();
        let repository = Arc::new(InMemoryParkedCarRepository::default());
        let checkin = CheckinUseCase::new(&working_hours, Arc::clone(&repository));
        let checkout = CheckoutUseCase::new(&working_hours, Arc::clone(&repository));

        let plate = "AAA9999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T16:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        checkin.perform(plate, checkin_date).await.unwrap();

        let checkout_date =
            NaiveDateTime::parse_from_str("2024-05-29T18:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let price = checkout.perform(plate, checkout_date).await;
        assert_eq!(price.unwrap(), 20.0);
    }

    // INFO: Teste de check-out de um carro não encontrado no sistema
    #[tokio::test]
    async fn test_checkout_car_not_found() {
        let working_hours = Period::new(
            NaiveDateTime::parse_from_str("2024-05-29T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )
        .unwrap();
        let repository = Arc::new(InMemoryParkedCarRepository::default());
        let checkout = CheckoutUseCase::new(&working_hours, Arc::clone(&repository));

        let plate = "AAA9999";
        let checkout_date =
            NaiveDateTime::parse_from_str("2024-05-29T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = checkout.perform(plate, checkout_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Parked car not found");
    }

    // INFO: Teste de check-in com uma placa inválida
    #[tokio::test]
    async fn test_invalid_plate() {
        let working_hours = Period::new(
            NaiveDateTime::parse_from_str("2024-05-29T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )
        .unwrap();
        let repository = Arc::new(InMemoryParkedCarRepository::default());
        let checkin = CheckinUseCase::new(&working_hours, Arc::clone(&repository));

        let plate = "AAA999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = checkin.perform(plate, checkin_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid plate");
    }

    // INFO: Teste de check-in antes da abertura do estacionamento
    #[tokio::test]
    async fn test_checkin_outside_open_hours() {
        let working_hours = Period::new(
            NaiveDateTime::parse_from_str("2024-05-29T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )
        .unwrap();
        let repository = Arc::new(InMemoryParkedCarRepository::default());
        let checkin = CheckinUseCase::new(&working_hours, Arc::clone(&repository));

        let plate = "AAA9999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T07:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = checkin.perform(plate, checkin_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid checkin date");
    }

    // INFO: Teste de check-in após o fechamento do estacionamento
    #[tokio::test]
    async fn test_checkin_outside_close_hours() {
        let working_hours = Period::new(
            NaiveDateTime::parse_from_str("2024-05-29T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
        )
        .unwrap();
        let repository = Arc::new(InMemoryParkedCarRepository::default());
        let checkin = CheckinUseCase::new(&working_hours, Arc::clone(&repository));

        let plate = "AAA9999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T22:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = checkin.perform(plate, checkin_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid checkin date");
    }
}
