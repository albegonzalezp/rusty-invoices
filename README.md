# Rusty Invoices

A professional command-line invoice management tool for self-employed professionals ("autónomos") in Spain. Generate, manage, and store professional invoices with ease.

## Features

- **Professional Invoice Generation**: Create and manage invoices with PDF generation
- **Client Management**: Store client information for quick invoice creation
- **Spanish Tax Compliance**: Apply IVA and IRPF calculations automatically
- **Input Validation**: Comprehensive validation for emails, CIF/NIE, dates, IBAN, and percentages
- **Cross-Platform**: Works on Linux, Windows, and macOS
- **Local Storage**: All data stored securely on your computer
- **Modern CLI**: Interactive menus with confirmation dialogs

## Quick Start

### Download

Download the latest release for your platform:

| Platform | Download |
|----------|----------|
| **Linux** | [rusty-invoices-linux-x86_64.tar.gz](https://github.com/albegonzalezp/rusty-invoices/releases/latest) |
| **Windows** | [rusty-invoices-windows-x86_64.zip](https://github.com/albegonzalezp/rusty-invoices/releases/latest) |
| **macOS Intel** | [rusty-invoices-macos-x86_64.tar.gz](https://github.com/albegonzalezp/rusty-invoices/releases/latest) |
| **macOS Apple Silicon** | [rusty-invoices-macos-aarch64.tar.gz](https://github.com/albegonzalezp/rusty-invoices/releases/latest) |

### Installation

#### Linux
```bash
# Extract the archive
tar -xzf rusty-invoices-linux-x86_64.tar.gz

# Make executable
chmod +x rusty-cli

# Run
./rusty-cli
```

#### Windows
```powershell
# Extract the zip file
Expand-Archive rusty-invoices-windows-x86_64.zip

# Run from Command Prompt or PowerShell
.\rusty-cli.exe
```

#### macOS
```bash
# Extract the archive
tar -xzf rusty-invoices-macos-x86_64.tar.gz  # Intel Macs
# or
tar -xzf rusty-invoices-macos-aarch64.tar.gz  # Apple Silicon

# Make executable
chmod +x rusty-cli

# Run
./rusty-cli
```

## Usage

### First Run

On first run, you'll be prompted to set up your profile:
- Name and surname
- CIF/NIE (Spanish tax identification number)
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
2. Choose a client from your saved clients
3. Configure invoice details:
   - Invoice number (auto-generated if empty)
   - Invoice date (defaults to today)
   - Due date (defaults to 30 days from invoice date)
4. Set tax rules:
   - **IVA percentage** (default: 21%)
   - **IRPF percentage** (default: 15%)
5. Add invoice items:
   - Description
   - Quantity
   - Price per unit
6. Generate professional PDF invoice

### Managing Data

- **List invoices**: View all created invoices
- **List clients**: Manage your client database
- **Update profile**: Modify your user information
- **Navigation**: "Go Back" options throughout the interface

## Data Storage

All data is stored locally on your computer:

| Platform | Location |
|----------|----------|
| **Linux** | `~/.rusty-invoices/` |
| **Windows** | `C:\Users\[user]\.rusty-invoices\` |
| **macOS** | `~/Library/Application Support/rusty-invoices/` |

```
rusty-invoices/
├── config.json          # Application configuration
├── user.json            # User profile
├── clients/             # Client data
│   └── *.json
├── invoices/            # Invoice data
│   └── *.json
└── pdfs/               # Generated PDFs
    └── *.pdf
```

## Configuration

Create `config.json` in your data directory:

```json
{
  "storage": {
    "base_path": "~/.rusty-invoices",
    "clients_dir": "clients",
    "invoices_dir": "invoices"
  },
  "pdf": {
    "output_dir": "~/.rusty-invoices/pdfs",
    "font_size": 12.0,
    "page_width": 210.0,
    "page_height": 297.0
  },
  "tax": {
    "default_iva": 21.0,
    "default_irpf": 15.0,
    "currency": "EUR"
  },
  "ui": {
    "welcome_message": "Welcome to Rusty Invoices",
    "confirm_prompts": true,
    "show_debug_info": false
  }
}
```

### Environment Variables

Override configuration with environment variables:

```bash
export RUSTY_INVOICES_TAX_DEFAULT_IVA=25.0
export RUSTY_INVOICES_UI_WELCOME_MESSAGE="Custom Welcome"
```

## Development

### Building from Source

#### Prerequisites
- Rust and Cargo (1.70.0 or newer)
- pkg-config (for PDF generation dependencies)

#### Build
```bash
# Clone the repository
git clone https://github.com/yourusername/rusty-invoices.git
cd rusty-invoices

# Build
cargo build --release

# Run
cargo run --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_user_creation

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

### Automated Testing
- **Multi-platform**: Linux, Windows, macOS
- **Multi-version**: stable, beta, nightly Rust
- **Quality checks**: formatting, clippy, tests, security audit
- **Cross-compilation**: Builds for all supported platforms

### Automated Releases
- **Triggered by**: Git tags (e.g., `v1.0.0`)
- **Artifacts**: Pre-compiled binaries for all platforms
- **Formats**: tar.gz and zip archives
- **Architectures**: x86_64 for all platforms, aarch64 for macOS

### Status Badges
[![CI](https://github.com/yourusername/rusty-invoices/workflows/CI/badge.svg)](https://github.com/yourusername/rusty-invoices/actions)
[![Security Audit](https://github.com/yourusername/rusty-invoices/workflows/Security%20Audit/badge.svg)](https://github.com/yourusername/rusty-invoices/actions)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation for new features
- Ensure all tests pass before submitting

## Example Invoice

[Download Example Invoice](images/invoice_102.pdf)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust for performance and safety
- PDF generation powered by `printpdf`
- CLI interface with `dialoguer` and `console`
- Configuration management with `config` crate
- Error handling with `thiserror` and `anyhow`

---

**Rusty Invoices** - Professional invoice management for Spanish freelancers 🇪🇸