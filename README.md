# Chat Application

This project is a simple chat application built in Rust. It interacts with a chat model API to send user messages and receive responses.

## Project Structure

```
chat-app
├── src
│   └── main.rs       # Contains the main function and chat logic
├── Cargo.toml        # Project configuration and dependencies
└── README.md         # Documentation for the project
```

## Dependencies

This project uses the following dependencies:

- `reqwest`: For making HTTP requests to the chat model API.
- `serde_json`: For handling JSON data.

## Setup

1. Ensure you have Rust installed on your machine. You can download it from [rust-lang.org](https://www.rust-lang.org/).

2. Clone this repository or download the project files.

3. Navigate to the project directory:

   ```
   cd chat-app
   ```

4. Build the project:

   ```
   cargo build
   ```

5. Run the application:

   ```
   cargo run
   ```

## Usage

- Start the chat application and follow the prompts.
- Type your message and press Enter to send it to the chat model.
- Type 'خروج' to exit the chat application.

## License

This project is licensed under the MIT License.