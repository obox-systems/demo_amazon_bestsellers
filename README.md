# Amazon bestsellers demo

This program will scrape the title, link, rating, and number of comments from the Amazon bestsellers page and save extracted data to a `bestsellers.csv` file.

It uses a Selenium / WebDriver method of extraction.

## Try it out!

1. Install [Rust](https://rustup.rs/)
2. Install [geckodriver](https://github.com/mozilla/geckodriver):
```bash
$ cargo install geckodriver
```
3. Optionally, on X11 you can install virtual framebuffer [Xvfb](https://en.wikipedia.org/wiki/Xvfb) to hide the puppet browser: 
```
# On Debian/Ubuntu
$ sudo apt install xvfb
# On Fedora
$ yum install xorg-x11-server-Xvfb
```
4. Run the app:
```bash
# This will start a virtual frame buffer :99, start geckodriver, compile, and run the scraper
$ ./scripts/start.sh
```

After the extraction's finished you'll find all extracted data in the `bestsellers.csv` file.
