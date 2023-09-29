# BD-Rust-Hackathon-Sept-2023

**Welcome to the first edition of the BD Rust Hackathon! Thanks for participating!**

Today, you're prompt is to make a barebones mock stock trading service as a POC for the `Tard Fi Data` company (a totally real company that I did not make up).

The repo you've been provided with contains a minimal web server that utilizes the tokio async runtime and the actix-web framework with a health check endpoint and database connection. Additionally, you'll see some crates already included in the `Cargo.toml` file that will help with things like logging. Additionally, `sqlx` has been installed to be the DB driver for the Postgres DB that is meant to be used to store data in.

Before we get into the prompt details, let's make sure your set up for success!

### Checklist
1. You have a laptop handy (duh)
2. You have an IDE installed (double duh)
3. Docker is installed
4. In your IDE, the `rust-analyzer` plug-in is running correctly
5. Have a GH account and can fork this repo
6. A database admin tool (any will do, as long as you can view a Postgres DB)


## Additional Set up

Before beginning, you'll need to make sure that you have a few things set up:

#### 1. Install sqlx-cli
```
cargo install sqlx-cli
```

This CLI will allow the `make run-migrations` command to set up the tables for you in the Postgres DB docker container.

### 2. Make sure you have a `.env` file stored locally with this variable
```
export DATABASE_URL="postgres://postgres:password@127.0.0.1:5432/tard_fi_data"
```

### Understand the make commands
1. `make start-db` will start a docker container for a Postgres DB
2. `make run-migrations` will run the migration stored in `/migrations` using the `sqlx` cli.
3. `make stop-db` will stop and delete the Postgres DB docker container

Try out these commands locally and make sure everything's working as expected

Your task today is to extend the existing server with mock functionality for stock trading:
- Create 4 new HTTP endpoints, each with some defined behavior for a GET and/or POST method:
    1. `/users`
        - GET: Return a json formatted list all users presently stored in the DB. names, emails, and user_ids should be returned.
        - POST: Accept a json payload with the name, email, and user ID (all string values) to create a new user. The `users` table should be updated with the provided info, as well as have a UUID and a `registered_at` timestamp. Here's an example payload:
        ```
            {
                "name": "Notro Bert",
                "email": "notrobert@gmail.com",
                "user_id" : "notrober2023"
            }
         ```
    2. `/price/{ticker}`
        - GET: Return the latest quote for a given stock ticker for the ticker sumbol provided in the path. Don't worry about rounding, return the entire float is fine.
        - Internally, the price data should be retrieved with the yahoo_finance_api crate (an example of it's use will be linked to).
    3. `/order/buy`
        - POST: Accept a json payload with the user_id you want to associate the transaction with, a valid stock ticker symbol (e.g. AAPL, AMZN, IBM, 3M, etc.), the quantity desired, whether or not the order is a limit order, and if a limit order, what your limit price is. You'll need to make sure the user_id, ticker symbol, price per share, total amount for the transaction, and the order type are recorded in the DB. Here's an example payload:
        ```
            {
                "user_id": "Notro Bert",
                "ticker": "AMZN",
                "quantity" : 30,
                "limit_order" : false
                "limit_price" : 200.00 // should be ignored unled limit_order is true
            }
         ```
        - Internally, the price data should be retrieved with the yahoo_finance_api crate (an example of it's use will be linked to). Use the latest quote to determing the price and to help determine the total amount.
        - If `limit_order` == false, attempt to mock the order and if successful, mark the `order_type` as `Market Buy`.
        - If `limit_order` == true, and the price is higher than `limit_price`, return an error to the user indicating the difference. Otherwise, mock the order and mark the `order_type` as `Limit Buy`.
        - If an invalid ticker is provided, return a message to the user indicating as such
    4. `/order/sell`
         - POST: Accept a json payload with the user_id you want to associate the transaction with, a valid stock ticker symbol (e.g. AAPL, AMZN, IBM, 3M, etc.), and the quantity of stocks. You'll need to make sure the user_id, ticker symbol, price per share, total amount for the transaction, and the order type are recorded in the DB. Here's an example payload:
        ```
            {
                "user_id": "Notro Bert",
                "ticker": "AMZN",
                "quantity" : 30,
                "limit_order" : false
                "limit_price" : 200.00 // should be ignored unled limit_order is true
            }
         ```
         - The price data will be retrieved in the same way it is for the buy endpoint.
        - If `limit_order` == false, attempt to mock the order and if successful, mark the `order_type` as `Market Sell`.
        - If `limit_order` == true, and the price is lower than `limit_price`, return an error to the user indicating the difference. Otherwise, mock the order and mark the `order_type` as `Limit Sell`.
         - If an invalid ticker is provided, return a message to the user indicating as such

    NOTE: We don't care about things like account balances for users - we just want to mock buy and sell orders

    That's it! You're free to play around with any part of the codebase, as long as the server behaves as expected in the end.
    Once you're done, submit public links or share your repo forks privately with `rjacksonxyz` on github.

    **REMEMBER**: The goal is **FUNCTIONALITY**. As long as the code works and you understand what you're doing, that's what's important. I'll share private feedback so that you can have a better idea of how to make your code idiomatic / workable.

    ## So what do I win?

    Details dropped in hackathon slack channel. Attendees who attend the opening ceremony and make their submission by 8:00p UTC are eligible!

    ## Helpful Bits

    If you plan on sticking with sqlx as the interface to interact with the DB, here's a really nice medium article: https://medium.com/@edandresvan/a-brief-introduction-about-rust-sqlx-5d3cea2e8544

    As promised, here's yahoo_finance_api documentation on getting the latest price quote: https://github.com/xemwebe/yahoo_finance_api/blob/master/examples/get_quote.rs
