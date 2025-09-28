# Rusty Invoices

A command-line invoice management tool designed for self-employed professionals ("autónomos") in Spain. Generate, manage, and store professional-looking invoices with ease.

## Features

- Create and manage professional invoices with PDF generation
- Store client information for quick invoice creation
- Apply Spanish tax rules (IVA, IRPF)
- Customizable invoice numbers, dates, and due dates
- Simple command-line interface for efficient workflow
- Local storage of all data and generated PDFs

## Installation

### Prerequisites

- Rust and Cargo (1.53.0 or newer)
- pkg-config (for PDF generation dependencies)

### Installing from source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rusty-cli.git
   cd rusty-cli
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

   Alternatively, you can install it to your system:
   ```bash
   cargo install --path .
   ```

## Usage

### First Run

On first run, you'll be prompted to set up your profile with the following information:
- Name and surname
- NIE/CIF (tax identification number)
- Address
- Email (optional)
- IBAN (optional)

### Creating Clients

1. Select "Create client" from the main menu
2. Enter client details:
   - Name
   - CIF (tax identification number)
   - Address
   - Email (optional)

### Creating Invoices

1. Select "Create invoice" from the main menu
2. Select a client from your saved clients
3. Optionally enter custom invoice details:
   - Invoice number (auto-generated if left empty)
   - Invoice date (defaults to today)
   - Due date (defaults to 30 days from invoice date)
4. Set tax rules:
   - IVA percentage (default: 21%)
   - IRPF percentage (default: 15%)
5. Add items to the invoice:
   - Description
   - Quantity
   - Price per unit
6. Generate a PDF of the invoice

### Managing Invoices

- List all invoices with "List invoices"
- View invoice details and generate PDFs of existing invoices
- All data is stored locally in `~/.rusty-invoices/`
- PDFs are saved in `~/.rusty-invoices/pdfs/`

## Data Storage

All data is stored locally on your computer:
- User profile: `~/.rusty-invoices/user.json`
- Clients: `~/.rusty-invoices/clients/`
- Invoices: `~/.rusty-invoices/invoices/`
- Generated PDFs: `~/.rusty-invoices/pdfs/`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request 

## Example Invoice

Below is an example of a generated invoice (PDF file):

[Download Example Invoice](images/invoice_102.pdf)

## Adding Images to README

To add images to this README, use the following Markdown syntax:

```markdown
![Alt text for the image](images/your-image.png "Optional title")
```

For screenshots or logos, store them in the `images/` directory and reference them as shown above. 