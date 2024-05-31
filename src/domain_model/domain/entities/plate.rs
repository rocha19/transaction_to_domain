#[cfg(test)]
mod plate_unit_test {
    use crate::domain_model::domain::models::plate::Plate;

    // INFO: Deve se a placa é válida;
    #[tokio::test]
    async fn test_if_plate_is_valid() {
        let plate = "AAA9999";

        let plate = Plate::new(plate);
        assert!(plate.is_ok());
    }

    // INFO: Deve testar se a placa é invalida;
    #[tokio::test]
    async fn test_if_plate_is_invalid() {
        let plate = "AAA999";

        let plate = Plate::new(plate);
        assert!(plate.is_err());
    }
}
