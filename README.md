# birthday

This repository hosts a simple yet delightful Birthday API built with Rust and GraphQL, offering a playful way to celebrate birthdays. With a touch of creativity, this app provides personalized birthday messages and countdowns to your special day.

## Installation

1. **Navigate to the project directory:**

    ```bash
    cd birthday
    ```

2. **Build and run the application:**

    ```bash
    cargo run
    ```

3. **Once the server is running, open your web browser and go to [http://127.0.0.1:8080/playground](http://127.0.0.1:8080/playground) to access the GraphQL playground.**

## Usage

- In the GraphQL playground, you can use the following query to check the birthday message:

    ```graphql
    {
      birthday {
        message
      }
    }
    ```

- Click the "Play" button to execute the query. The response will display the birthday message based on the current date.

## Customization

- If you'd like to customize the birthday message, you can do so by modifying the code in the `main.rs` file. Look for the `birthday` function in the `Query` struct and update the message strings to your liking.

## Contributing

- Feel free to contribute to this project by opening issues or creating pull requests. Your contributions are highly appreciated!

## Author

[Webster Avosa](https://github.com/avosa)

Enjoy!