#[cfg(test)]
mod parked_car_unit_test {
    use chrono::NaiveDateTime;
    use value_object::domain_model::domain::entities::parked_car::ParkedCar;

    // INFO: Deve testar um carro estacionado;
    #[tokio::test]
    async fn test_if_car_is_parked() {
        let plate = "AAA9999".to_string();
        let checkin_date =
            NaiveDateTime::parse_from_str("2024-05-29T16:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        let mut parked_car =
            ParkedCar::new(plate, checkin_date).expect("Failed to create parked car");

        let checkout_date =
            NaiveDateTime::parse_from_str("2024-05-29T18:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();

        parked_car.checkout(checkout_date);
        let price = parked_car
            .price
            .expect("Price should be calculated after checkout");
        assert_eq!(price, 20.0);
    }
}
