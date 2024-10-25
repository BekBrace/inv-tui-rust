// Hey, what's going on everyone? Welcome back to the BackBrace channel! My name is Amir, and I’m excited to have you here today. In this tutorial, we’re going to build a robust inventory management system using Rust and the Cursive library. With this system, you’ll be able to easily store, view, and delete items, making it a handy tool for anyone who needs to keep track of inventory!

// Before we dive into the code, I want to take a moment to thank all of you for your amazing support. Whether you’re leaving comments, hitting that like button, or just tuning in to watch my videos, it means the world to me! Your engagement helps the channel grow and motivates me to create more content. If you’re new here, don’t forget to subscribe and turn on notifications so you can stay updated with our latest projects.

// Today’s project will cover key concepts in Rust programming, including how to work with structs, serialization, and using a user-friendly interface with Cursive. So grab your coffee, open your vscode and let’s get started!


// Before we start coding, I want to say This app is multi-threaded, as it uses Arc and Mutex from the Rust standard library. The Arc (Atomic Reference Counting) allows multiple threads to have shared ownership of the data (in this case, the inventory products), and Mutex ensures that only one thread can access the data at a time to prevent race conditions. So, although the code itself doesn’t explicitly spawn new threads, it's structured in a way that allows safe concurrent access, making it thread-safe and ready for multi-threading if you decide to extend it further.

// Import necessary modules from the Cursive library for creating a text-based UI.
use cursive::views::{Dialog, EditView, ListView}; // Import Dialog, EditView, and ListView for the UI components.
use cursive::{Cursive, CursiveExt}; // Import Cursive for creating and managing the application.
use cursive::traits::{Nameable, Resizable}; // Import traits for naming and resizing UI components.

use std::sync::{Arc, Mutex}; // Import Arc and Mutex for thread-safe shared ownership.
use std::fs::{File, OpenOptions}; // Import File and OpenOptions for file operations.
use std::io::{self, Read}; // Import io module and the Read trait for reading from files.
use serde::{Serialize, Deserialize}; // Import traits for serializing and deserializing data.

// Explanation of derive in seperate file
#[derive(Debug, Clone, Serialize, Deserialize)] // Derive Debug for debugging, Clone for cloning instances, Serialize and Deserialize for JSON conversion.
struct Product { // Define a struct to represent a product in the inventory.
    product_type: String, // Product type (e.g., "Electronics", "Clothing").
    quantity: usize, // Quantity of the product in stock.
    price_per_unit: f64, // Price of each unit of the product.
    sales_tax: f64, // Sales tax applied to the product.
    total_price: f64, // Total price after tax for the quantity of the product.
}

const FILE_PATH: &str = "inventory.json"; // Define a constant for the file path where inventory data will be stored.

fn save_products_to_file(products: &Vec<Product>) -> io::Result<()> { // Function to save products to a JSON file.
    let file = OpenOptions::new() // Create a new OpenOptions instance to configure file opening.
        .write(true) // Specify that we want to write to the file.
        .create(true) // Create the file if it doesn't exist.
        .truncate(true) // Truncate the file to zero length before writing.
        .open(FILE_PATH)?; // Open the file at the specified path, handling any errors.
    
    serde_json::to_writer(file, products)?; // Serialize the products vector to JSON and write it to the file.
    Ok(()) // Return Ok to indicate success.
}

fn load_products_from_file() -> Vec<Product> { // Function to load products from the JSON file.
    if let Ok(mut file) = File::open(FILE_PATH) { // Try to open the file.
        let mut data = String::new(); // Create a new string to hold the file data.
        if file.read_to_string(&mut data).is_ok() { // Read the file content into the string.
            if let Ok(products) = serde_json::from_str::<Vec<Product>>(&data) { // Try to deserialize the string into a vector of Product structs.
                return products; // Return the loaded products if successful.
            }
        }
    }
    Vec::new() // Return an empty vector if the file doesn't exist or deserialization fails.
}

fn main() { // Main function where the application starts.
    let mut siv = Cursive::default(); // Create a new instance of Cursive for the UI.

    let products = Arc::new(Mutex::new(load_products_from_file())); // Load products from file and wrap them in Arc and Mutex for safe shared access.

    // Add a dialog layer to the UI for managing the inventory.
    siv.add_layer(
        Dialog::new() // Create a new dialog.
            .title("Inventory Management") // Set the dialog title.
            .content(ListView::new() // Set the dialog content to a new ListView.
                .child("Product Type:", EditView::new().with_name("product_type")) // Add an EditView for entering the product type.
                .child("Quantity:", EditView::new().with_name("quantity")) // Add an EditView for entering the quantity.
                .child("Price per Unit:", EditView::new().with_name("price_per_unit")) // Add an EditView for entering the price per unit.
            )
            .button("Save", { // Add a button to save the product.
                let products_clone = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| { // Closure that runs when the button is pressed.
                    let product_type = s // Get the content from the EditView named "product_type".
                        .call_on_name("product_type", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() // Unwrap the result, panicking if there’s an error.
                        .to_string(); // Convert the content to a String.

                    let quantity = s // Get the content from the EditView named "quantity".
                        .call_on_name("quantity", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() // Unwrap the result.
                        .parse::<usize>() // Parse the content as usize.
                        .unwrap_or(0); // If parsing fails, default to 0.

                    let price_per_unit = s // Get the content from the EditView named "price_per_unit".
                        .call_on_name("price_per_unit", |view: &mut EditView| {
                            view.get_content()
                        })
                        .unwrap() // Unwrap the result.
                        .parse::<f64>() // Parse the content as f64.
                        .unwrap_or(0.0); // If parsing fails, default to 0.0.

                    // Validation: Check if the fields are empty or invalid.
                    if product_type.is_empty() { // Check if the product type is empty.
                        s.add_layer(Dialog::info("Error: Please enter a product type.")); // Show an error dialog.
                        return; // Exit the closure.
                    }

                    if quantity == 0 { // Check if the quantity is invalid.
                        s.add_layer(Dialog::info("Error: Please enter a valid quantity.")); // Show an error dialog.
                        return; // Exit the closure.
                    }

                    if price_per_unit == 0.0 { // Check if the price per unit is invalid.
                        s.add_layer(Dialog::info("Error: Please enter a valid price.")); // Show an error dialog.
                        return; // Exit the closure.
                    }

                    let sales_tax = 0.10 * price_per_unit; // Calculate sales tax at a rate of 10%.
                    let total_price = (price_per_unit + sales_tax) * quantity as f64; // Calculate total price.

                    let product = Product { // Create a new Product instance.
                        product_type,
                        quantity,
                        price_per_unit,
                        sales_tax,
                        total_price,
                    };

                    let mut product_store = products_clone.lock().unwrap(); // Lock the Mutex to safely access the products.
                    product_store.push(product.clone()); // Add the new product to the product store.

                    // Save to file
                    if let Err(err) = save_products_to_file(&product_store) { // Try to save the products to file.
                        s.add_layer(Dialog::info(format!("Error saving product: {}", err))); // Show an error dialog if saving fails.
                    } else {
                        s.add_layer(Dialog::info("Product saved successfully!")); // Show a success dialog.
                    }
                }
            })
            .button("Show All", { // Add a button to show all products.
                let products = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| { // Closure that runs when the button is pressed.
                    let product_store = products.lock().unwrap(); // Lock the Mutex to access the products.
                    let mut output = String::new(); // Create a string to hold the output.

                    for (index, product) in product_store.iter().enumerate() { // Iterate through each product.
                        output.push_str(&format!( // Format the product details into the output string.
                            "{}. Item: {}, Qty: {}, Price: ${}, Sales Tax: ${}, T.Price: ${}\n",
                            index + 1, // Product index (1-based).
                            product.product_type, // Product type.
                            product.quantity, // Quantity.
                            product.price_per_unit, // Price per unit.
                            product.sales_tax, // Sales tax.
                            product.total_price // Total price.
                        ));
                    }

                    if output.is_empty() { // Check if there are no products.
                        output = "No products in the inventory.".to_string(); // Set a message if there are no products.
                    }

                    s.add_layer(Dialog::info(output)); // Show the output in a dialog.
                }
            })
            .button("Delete by ID", { // Add a button to delete a product by ID.
                let products_clone = Arc::clone(&products); // Clone the Arc for thread-safe access.
                move |s| { // Closure that runs when the button is pressed.
                    // Get ID from user
                    let id_input = EditView::new().with_name("delete_id").min_width(10); // Create an EditView for entering the product ID.
                    s.add_layer(Dialog::new() // Create a new dialog for deleting a product.
                        .title("Delete Product") // Set the dialog title.
                        .content(ListView::new() // Set the content of the dialog.
                            .child("Enter product ID to delete:", id_input) // Add the ID input field.
                        )
                        .button("Confirm", { // Add a button to confirm deletion.
                            let products_clone = Arc::clone(&products_clone); // Clone the Arc for thread-safe access.
                            move |s| { // Closure that runs when the button is pressed.
                                let id_str = s // Get the content from the EditView named "delete_id".
                                    .call_on_name("delete_id", |view: &mut EditView| {
                                        view.get_content()
                                    })
                                    .unwrap() // Unwrap the result.
                                    .to_string(); // Convert the content to a String.

                                // Parse ID
                                if let Ok(id) = id_str.parse::<usize>() { // Try to parse the ID as usize.
                                    let mut product_store = products_clone.lock().unwrap(); // Lock the Mutex to access the products.

                                    // Check if ID is valid
                                    if id > 0 && id <= product_store.len() { // Check if the ID is within the valid range.
                                        product_store.remove(id - 1); // Remove the product from the store (adjusting for 0-based index).
                                        if let Err(err) = save_products_to_file(&product_store) { // Try to save the updated products to file.
                                            s.add_layer(Dialog::info(format!("Error deleting product: {}", err))); // Show an error dialog if saving fails.
                                        } else {
                                            s.add_layer(Dialog::info("Product deleted successfully!")); // Show a success dialog.
                                        }
                                    } else {
                                        s.add_layer(Dialog::info("Error: Invalid product ID.")); // Show an error dialog if the ID is invalid.
                                    }
                                } else {
                                    s.add_layer(Dialog::info("Error: Please enter a valid number.")); // Show an error dialog if the ID is not a valid number.
                                }
                            }
                        })
                        .button("Cancel", |s| { // Add a button to cancel the deletion.
                            s.pop_layer(); // Remove the delete dialog layer.
                        })
                    );
                }
            })
            .button("Quit", |s| s.quit()), // Add a button to quit the application.
    );

    siv.run(); // Run the Cursive event loop, starting the application.
}
