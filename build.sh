if [ -z "$1" ]; then
    cargo objcopy --release --bin blink -- -O ihex blink.hex
else
    cargo objcopy --release --bin $1 -- -O ihex "$1.hex"
fi
