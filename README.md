# Amazon bestsellers demo

This program will scrape the title, link, rating and number of comments from Amazon bestsellers page and save extracted data to a `bestsellers.csv` file.

It uses a Selenium / WebDriver method of extraction.

## Try it out!

1. Install [Rust](https://rustup.rs/)
2. Install [geckodriver](https://github.com/mozilla/geckodriver):
```bash
$ cargo install geckodriver
```
3. Run geckodriver:
```bash
$ geckodriver
```
4. In another terminal run:
```bash
$ cargo run 
```

After the exctraction is finished the pupet browser will close and you'll find all extracted data in the `bestsellers.csv` file.
