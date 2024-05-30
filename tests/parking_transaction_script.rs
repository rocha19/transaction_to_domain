#[cfg(test)]
mod parking_transaction_script {
    use std::time::{Duration, UNIX_EPOCH};
    use value_object::application::transaction_script::parking_service::ParkingService;

    #[tokio::test]
    async fn test_valid_checkin_and_checkout() {
        let plate = "AAA9999".to_string();
        let mut parking_service = ParkingService::new();

        // Set the check-in time
        let checkin_date = UNIX_EPOCH + Duration::from_secs(4 * 3600); // 4 hours since UNIX_EPOCH
        parking_service
            .checkin(plate.clone(), checkin_date)
            .await
            .unwrap();

        // Set the check-out time
        let checkout_date = UNIX_EPOCH + Duration::from_secs(6 * 3600); // 6 hours since UNIX_EPOCH
        let price = parking_service
            .checkout(&plate, checkout_date)
            .await
            .unwrap();

        assert_eq!(price, 20.0);
    }

    // INFO: Não lançar uma erro carro não encontrado tente sair
    #[tokio::test]
    async fn test_is_plate_not_found_error() {
        let plate = "AAA9999".to_string();
        let mut parking_service = ParkingService::new();
        let checkout_date = UNIX_EPOCH + Duration::from_secs(6 * 3600); // 6 hours since UNIX_EPOCH
        let result = parking_service.checkout(&plate, checkout_date).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Car not found".to_string());
    }

    // INFO: Não deve deve entrar carro com plana inválida
    #[tokio::test]
    async fn test_invalid_plate_error() {
        let invalid_plate = "AA99".to_string();
        let mut parking_service = ParkingService::new();

        let checkin_date = UNIX_EPOCH + Duration::from_secs(4 * 3600); // 4 hours since UNIX_EPOCH
        let result = parking_service
            .checkin(invalid_plate.clone(), checkin_date)
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid plate".to_string());
    }
}
