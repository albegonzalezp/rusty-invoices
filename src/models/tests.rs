#[cfg(test)]
mod model_tests {
    use crate::models::{Client, Invoice, Item, Rule, User};

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "12345678A".to_string(),
            Some("john@example.com".to_string()),
            Some("ES1234567890123456789012".to_string()),
        );

        assert_eq!(user.name, "John Doe");
        assert_eq!(user.address, "123 Main St");
        assert_eq!(user.cif, "12345678A");
        assert_eq!(user.email, Some("john@example.com".to_string()));
        assert_eq!(user.iban, Some("ES1234567890123456789012".to_string()));
    }

    #[test]
    fn test_item_creation() {
        let item = Item::new("Web Development".to_string(), 1, 1000.0);

        assert_eq!(item.description, "Web Development");
        assert_eq!(item.quantity, 1);
        assert_eq!(item.price, 1000.0);
        assert_eq!(item.total(), 1000.0);
    }

    #[test]
    fn test_rule_creation() {
        let rule = Rule::new(21.0, 15.0);

        assert_eq!(rule.iva, 21.0);
        assert_eq!(rule.irpf, 15.0);
    }

    #[test]
    fn test_invoice_calculations() {
        let user = User::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "12345678A".to_string(),
            None,
            None,
        );

        let client = Client::new(
            "Acme Corp".to_string(),
            "98765432C".to_string(),
            "789 Business Blvd".to_string(),
            None,
        );

        let rule = Rule::new(21.0, 15.0);
        let item = Item::new("Web Development".to_string(), 1, 1000.0);
        let items = vec![item];

        let invoice = Invoice::new(
            "INV-001".to_string(),
            "2024-01-01".to_string(),
            "2024-01-31".to_string(),
            user,
            client,
            rule,
            items,
        );

        assert_eq!(invoice.id, "INV-001");
        assert_eq!(invoice.subtotal, 1000.0);
        assert_eq!(invoice.iva_amount, 210.0); // 21% of 1000
        assert_eq!(invoice.irpf_amount, 150.0); // 15% of 1000
        assert_eq!(invoice.total, 1060.0); // 1000 + 210 - 150
    }
}
