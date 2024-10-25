
# Inventory Management System

A simple, text-based Inventory Management application built with Rust and the Cursive library. This tool allows users to add, view, and delete products in an inventory, with persistent data stored in a JSON file.

## Features
- **Add Products**: Add products with details like type, quantity, price per unit, sales tax, and total price.
- **View Inventory**: List all inventory items with calculated sales tax and total prices.
- **Delete by ID**: Remove a specific product from the inventory.
- **Persistent Storage**: Data is saved in `inventory.json` to retain products between sessions.

## Technologies
- **Rust**: For the application logic and data handling.
- **Cursive**: For the text-based UI.
- **Serde**: For JSON serialization and deserialization.
- **Arc and Mutex**: For safe, thread-safe shared ownership.

## Getting Started

### Prerequisites
Ensure that **Rust** is installed. You can install Rust by following the official guide [here](https://www.rust-lang.org/tools/install).

### Installation
1. **Clone the Repository:**
   ```bash
   git clone 'put the link here'
   cd inventory-management
   ```

2. **Install Dependencies:**
   Install the required dependencies using Cargo:
   ```bash
   cargo build
   ```

3. **Run the Application:**
   Start the inventory management system:
   ```bash
   cargo run
   ```

## Usage Guide

The application provides a simple, text-based interface. Follow these steps to manage your inventory:

1. **Add Product**: Enter product type, quantity, and price per unit. Sales tax and total price are calculated automatically.
2. **Show All**: View all inventory items with their respective details.
3. **Delete by ID**: Enter the ID of the product you wish to delete.
4. **Quit**: Exit the application.

## Code Structure

- **Product Struct**: Defines a product with type, quantity, price per unit, sales tax, and total price.
- **File Operations**: JSON serialization and deserialization for saving/loading products to/from `inventory.json`.
- **Cursive UI**: Provides a text-based user interface for product management tasks.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
