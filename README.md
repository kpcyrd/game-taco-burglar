# game-taco-burglar

This is memory-safe firmware for a handheld I built for a friends birthday.

The original device I gave away with firmware tagged as `v1.0.0`.

## Bill of materials

- waveshare-rp2040-zero
- ssd1306
- sh1106
- 3 buttons (two on the left, one on the right)

## Build instructions

```
git clone https://github.com/kpcyrd/game-taco-burglar
cd game-taco-burglar
rustup target add thumbv6m-none-eabi
cargo build --release
# Flash to device
elf2uf2-rs -d target/thumbv6m-none-eabi/release/game-taco-burglar
```
