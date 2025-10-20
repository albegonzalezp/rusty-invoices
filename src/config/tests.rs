#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();

        assert_eq!(config.tax.default_iva, 21.0);
        assert_eq!(config.tax.default_irpf, 15.0);
        assert_eq!(config.tax.currency, "EUR");
        assert_eq!(config.pdf.font_size, 12.0);
        assert_eq!(config.pdf.page_width, 210.0);
        assert_eq!(config.pdf.page_height, 297.0);
        assert_eq!(config.ui.welcome_message, "Welcome to Rusty Invoices");
        assert!(config.ui.confirm_prompts);
        assert!(!config.ui.show_debug_info);
    }

    #[test]
    fn test_config_validation_valid() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_iva() {
        let mut config = AppConfig::default();
        config.tax.default_iva = 150.0; // Invalid: > 100

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_irpf() {
        let mut config = AppConfig::default();
        config.tax.default_irpf = -10.0; // Invalid: < 0

        assert!(config.validate().is_err());
    }
}
