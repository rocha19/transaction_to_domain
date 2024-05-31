#[cfg(test)]
mod parking_dao {
    use chrono::NaiveDateTime;
    use value_object::dao::{
        in_memory_parked_car_dao::ParkedCarDAOInMemory, parked_car_service::ParkingServiceDAO,
    };

    #[tokio::test]
    async fn test_checkin_checkout() {
        let dao = ParkedCarDAOInMemory::new();
        let service = ParkingServiceDAO::new(dao);

        let plate = "AAA9999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T16:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        service.checkin(plate, checkin_date).await.unwrap();

        let checkout_date =
            NaiveDateTime::parse_from_str("2024-05-29T18:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let price = service.checkout(plate, checkout_date).await.unwrap();
        assert_eq!(price, 20.0);
    }

    #[tokio::test]
    async fn test_checkout_car_not_found() {
        let dao = ParkedCarDAOInMemory::new();
        let service = ParkingServiceDAO::new(dao);

        let plate = "AAA9999";
        let checkout_date =
            NaiveDateTime::parse_from_str("2024-05-29T10:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = service.checkout(plate, checkout_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Parked car not found");
    }

    #[tokio::test]
    async fn test_invalid_plate() {
        let dao = ParkedCarDAOInMemory::new();
        let service = ParkingServiceDAO::new(dao);

        let plate = "AAA999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T14:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = service.checkin(plate, checkin_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid plate");
    }

    // INFO:  Não deve entrar carro antes da abertura do estacionamento
    #[tokio::test]
    async fn test_checkin_outside_open_hours() {
        let dao = ParkedCarDAOInMemory::new();
        let service = ParkingServiceDAO::new(dao);

        let plate = "AAA9999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T07:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = service.checkin(plate, checkin_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Parking lot is closed");
    }
    // INFO:  Não deve entrar após o fechamento do estacionamento
    #[tokio::test]
    async fn test_checkin_outside_close_hours() {
        let dao = ParkedCarDAOInMemory::new();
        let service = ParkingServiceDAO::new(dao);

        let plate = "AAA9999";
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T22:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let result = service.checkin(plate, checkin_date).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Parking lot is closed");
    }
}
