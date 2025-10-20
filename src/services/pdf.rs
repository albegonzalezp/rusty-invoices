// PDF Service for generating invoice PDFs

use crate::models::Invoice;
use printpdf::*;
use std::fs::File;
use std::io::{self, BufWriter};

// Service for generating PDF invoices
#[derive(Clone)]
pub struct PdfService {
    output_dir: String,  // Directory where generated PDFs will be stored
}

impl PdfService {
    pub fn new(output_dir: String) -> io::Result<Self> {
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| io::Error::new(io::ErrorKind::PermissionDenied, 
                format!("Failed to create PDF output directory '{}': {}", output_dir, e)))?;
        Ok(PdfService { output_dir })
    }

    // Generate a PDF invoice from an Invoice model
    pub fn generate_invoice_pdf(&self, invoice: &Invoice) -> io::Result<String> {
        // Create a PDF document with A4 dimensions
        let (doc, page1, layer1) = PdfDocument::new("Invoice", Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Load fonts
        let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, 
                format!("Failed to load regular font: {}", e)))?;
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, 
                format!("Failed to load bold font: {}", e)))?;
        let font_italic = doc.add_builtin_font(BuiltinFont::HelveticaOblique)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, 
                format!("Failed to load italic font: {}", e)))?;

        // Define colors
        let blue_color = printpdf::Color::Rgb(Rgb::new(0.0, 0.35, 0.7, None));
        let black_color = printpdf::Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
        let gray_color = printpdf::Color::Rgb(Rgb::new(0.5, 0.5, 0.5, None));
        let light_gray_color = printpdf::Color::Rgb(Rgb::new(0.95, 0.95, 0.95, None));

        // Add header with title
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "INVOICE",
            24.0,
            Mm(30.0),
            Mm(270.0),
            blue_color.clone(),
        );

        // Add invoice details (no box)
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "INVOICE #",
            11.0,
            Mm(145.0),
            Mm(275.0),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &invoice.id,
            11.0,
            Mm(145.0),
            Mm(270.0),
            black_color.clone(),
        );

        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "DATE",
            11.0,
            Mm(145.0),
            Mm(260.0),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &invoice.date,
            11.0,
            Mm(145.0),
            Mm(255.0),
            black_color.clone(),
        );

        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "DUE DATE",
            11.0,
            Mm(145.0),
            Mm(245.0),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &invoice.due_date,
            11.0,
            Mm(145.0),
            Mm(240.0),
            black_color.clone(),
        );

        // Add issuer details
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "FROM",
            14.0,
            Mm(30.0),
            Mm(240.0),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            &invoice.user.name,
            12.0,
            Mm(30.0),
            Mm(235.0),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &format!("CIF/NIE: {}", invoice.user.cif),
            10.0,
            Mm(30.0),
            Mm(230.0),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &invoice.user.address,
            10.0,
            Mm(30.0),
            Mm(225.0),
            black_color.clone(),
        );
        
        if let Some(email) = &invoice.user.email {
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &format!("Email: {}", email),
                10.0,
                Mm(30.0),
                Mm(220.0),
                black_color.clone(),
            );
        }
        
        if let Some(iban) = &invoice.user.iban {
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &format!("IBAN: {}", iban),
                10.0,
                Mm(30.0),
                Mm(215.0),
                black_color.clone(),
            );
        }

        // Add client details
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "BILL TO",
            14.0,
            Mm(30.0),
            Mm(200.0),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            &invoice.client.name,
            12.0,
            Mm(30.0),
            Mm(195.0),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &format!("CIF/NIF: {}", invoice.client.cif),
            10.0,
            Mm(30.0),
            Mm(190.0),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &invoice.client.address,
            10.0,
            Mm(30.0),
            Mm(185.0),
            black_color.clone(),
        );
        
        if let Some(email) = &invoice.client.email {
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &format!("Email: {}", email),
                10.0,
                Mm(30.0),
                Mm(180.0),
                black_color.clone(),
            );
        }

        // Add table header
        let table_y = 165.0;
        
        // Simple header with bold text
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "DESCRIPTION",
            10.0,
            Mm(35.0),
            Mm(table_y),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "QTY",
            10.0,
            Mm(100.0),
            Mm(table_y),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "PRICE",
            10.0,
            Mm(120.0),
            Mm(table_y),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "TOTAL",
            10.0,
            Mm(150.0),
            Mm(table_y),
            blue_color.clone(),
        );

        // Draw a line under the header
        self.draw_line(
            &current_layer,
            Mm(30.0),
            Mm(table_y - 2.0),
            Mm(180.0),
            Mm(table_y - 2.0),
            0.5,
            gray_color.clone(),
        );

        // Add item rows
        let mut y_position = table_y - 8.0; // Reduced spacing
        for (i, item) in invoice.items.iter().enumerate() {
            // Simplify description handling - just truncate if too long
            let description = if item.description.len() > 40 {
                format!("{}...", &item.description[0..37])
            } else {
                item.description.clone()
            };
            
            // Add description
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &description,
                9.0,
                Mm(35.0),
                Mm(y_position),
                black_color.clone(),
            );
            
            // Add other item details
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &item.quantity.to_string(),
                9.0,
                Mm(100.0),
                Mm(y_position),
                black_color.clone(),
            );
            
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &format!("{:.2} €", item.price),
                9.0,
                Mm(120.0),
                Mm(y_position),
                black_color.clone(),
            );
            
            self.add_text_with_color(
                &current_layer,
                &font_regular,
                &format!("{:.2} €", item.total()),
                9.0,
                Mm(150.0),
                Mm(y_position),
                black_color.clone(),
            );
            
            // Draw a light separator line between items (optional)
            if i < invoice.items.len() - 1 {
                self.draw_line(
                    &current_layer,
                    Mm(35.0),
                    Mm(y_position - 2.0),
                    Mm(175.0),
                    Mm(y_position - 2.0),
                    0.2,
                    light_gray_color.clone(),
                );
            }
            
            // Move to next row position with reduced spacing
            y_position -= 7.0; // Reduced from 10.0 to 7.0
        }

        // Draw a line after items
        self.draw_line(
            &current_layer,
            Mm(30.0),
            Mm(y_position + 4.0),
            Mm(180.0),
            Mm(y_position + 4.0),
            0.5,
            gray_color.clone(),
        );

        y_position -= 4.0; // Reduced spacing
        
        // Add summary
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "SUBTOTAL:",
            10.0,
            Mm(100.0),
            Mm(y_position),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &format!("{:.2} €", invoice.subtotal),
            10.0,
            Mm(150.0),
            Mm(y_position),
            black_color.clone(),
        );
        
        y_position -= 5.0; // Reduced spacing
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            &format!("IVA ({}%):", invoice.rule.iva),
            10.0,
            Mm(100.0),
            Mm(y_position),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &format!("{:.2} €", invoice.iva_amount),
            10.0,
            Mm(150.0),
            Mm(y_position),
            black_color.clone(),
        );
        
        y_position -= 5.0; // Reduced spacing
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            &format!("IRPF ({}%):", invoice.rule.irpf),
            10.0,
            Mm(100.0),
            Mm(y_position),
            black_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_regular,
            &format!("-{:.2} €", invoice.irpf_amount),
            10.0,
            Mm(150.0),
            Mm(y_position),
            black_color.clone(),
        );
        
        y_position -= 8.0; // Reduced spacing
        
        // Draw a line above total
        self.draw_line(
            &current_layer,
            Mm(100.0),
            Mm(y_position + 3.0),
            Mm(180.0),
            Mm(y_position + 3.0),
            1.0,
            black_color.clone(),
        );
        
        y_position -= 5.0; // Reduced spacing
        
        // Total amount with black text (no background)
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            "TOTAL:",
            16.0,
            Mm(100.0),
            Mm(y_position),
            blue_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_bold,
            &format!("{:.2} €", invoice.total),
            16.0,
            Mm(150.0),
            Mm(y_position),
            blue_color.clone(),
        );
        
        // Add footer
        let footer_y = 30.0;
        self.draw_line(
            &current_layer,
            Mm(30.0),
            Mm(footer_y + 5.0),
            Mm(180.0),
            Mm(footer_y + 5.0),
            0.5,
            gray_color.clone(),
        );
        
        self.add_text_with_color(
            &current_layer,
            &font_italic,
            "Thank you for your business!",
            10.0,
            Mm(105.0),
            Mm(footer_y),
            gray_color.clone(),
        );

        // Save the PDF
        let output_path = format!("{}/invoice_{}.pdf", self.output_dir, invoice.id);
        
        // Create file and handle errors
        let file = File::create(&output_path)?;
        let mut writer = BufWriter::new(file);
        
        // Save the PDF and convert any errors to io::Error
        doc.save(&mut writer).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("PDF generation error: {}", e))
        })?;

        Ok(output_path)
    }

    // Helper method to add text with a specific color
    fn add_text_with_color(
        &self,
        layer: &PdfLayerReference,
        font: &IndirectFontRef,
        text: &str,
        size: f32,
        x: Mm,
        y: Mm,
        color: printpdf::Color,
    ) {
        layer.set_fill_color(color);
        layer.use_text(text, size, x, y, font);
    }
    
    // Helper method to draw a line
    fn draw_line(
        &self,
        layer: &PdfLayerReference,
        x1: Mm,
        y1: Mm,
        x2: Mm,
        y2: Mm,
        thickness: f32,
        color: printpdf::Color,
    ) {
        layer.set_outline_thickness(thickness);
        layer.set_outline_color(color);
        
        // Create a line from point 1 to point 2
        let line = Line {
            points: vec![(Point::new(x1, y1), false), (Point::new(x2, y2), false)],
            is_closed: false,
        };
        
        // Draw the line
        layer.add_line(line);
    }
}
