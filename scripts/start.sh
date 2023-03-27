killall geckodriver Xvfb 2> /dev/null

# Virtual display to hide the puppet browser
Xvfb :99 -ac & 

# Firefox driver
DISPLAY=:99 MOZ_REMOTE_SETTINGS_DEVTOOLS=1 geckodriver 2> /dev/null | grep INFO &

# Run scraper
cargo run --release

# Cleanup
killall geckodriver Xvfb