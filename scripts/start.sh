killall geckodriver Xvfb 2> /dev/null

# Virtual display to hide the puppet browser
Xvfb :99 -ac & 
set DISPLAY=:99

# Firefox driver
set MOZ_REMOTE_SETTINGS_DEVTOOLS=1
geckodriver 2> /dev/null | grep INFO &

# Run scraper
cargo run --release

# Cleanup
killall geckodriver Xvfb