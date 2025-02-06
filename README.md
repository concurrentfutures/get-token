

# Discord Token Fetche

This project is a Rust-based implementation of a Discord Token Fetcher that allows you to retrieve a Discord token using your email and password.

## Features

- Fetch Discord token using email and password.
- Handles different error responses:
  - Invalid password.
  - Captcha required.
  - General errors.
- Clean terminal output with color formatting using ANSI escape codes.
- Pauses the terminal after completing the fetch process.

# Requirements

- Rust (Installation instructions can be found [here](https://www.rust-lang.org/tools/install)).
- `reqwest`, `serde`, and `serde_json` dependencies for making HTTP requests and handling JSON.

# Setup

### 1. Clone the repository:

```bash
git clone https://github.com/your-username/discord-token-fetcher-rust.git
cd discord-token-fetcher-rust


2. Add dependencies:

In your Cargo.toml file, add the following dependencies:

[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

3. Build the project:

Run the following command to build the project:

cargo build --release

4. Run the script:

After building, run the executable:

cargo run

The program will prompt you to enter your email and password, and it will attempt to fetch the token from Discord. It will display the token if successful or show an error message if something goes wrong.

Usage

1. When prompted, enter your Discord account's email and password.


2. The script will attempt to fetch the token.


3. The token (or an error message) will be displayed in the terminal.



Error Handling

Invalid Password: If the password doesn't match the account, an error message will be displayed.

Captcha Required: If Discord requires a captcha, the script will inform you and advise you to try again later.

General Error: If any other issue occurs, a general error message will be shown.


Example Output

> Email: example@example.com
> Password: ********

> Token: <discord-token-here>
> Successfully Fetched Token

Or in case of an error:

> Invalid Password

License

This project is licensed under the MIT License - see the LICENSE file for details.

Disclaimer

This tool is for educational purposes only. Misuse of this tool to violate Discord's Terms of Service or any other platform's policies is strictly prohibited.
