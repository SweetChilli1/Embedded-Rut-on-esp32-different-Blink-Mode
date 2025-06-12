# Embedded-Rut-on-esp32-different-Blink-Mode🦀

This is my first small Rust program for the esp32. 
## Setup:
It consists of an ESP32, 3 buttons and on LED. Each button has it's own mode:

|**Element** | **Gpio**      | **Function**
|-----------|----|----------------------------------------------------------------------------------------------------------------------------------------------------------------------
| **Led** | 13 | Show the output
| **Button 1 (white)** | 4 | Led lights up as long as the button is pressed
| **Button 2 (red)** | 21 | Led flashes with an delay of 500 milliseconds as long as the button is pressed
| **Button 3 (blue)** | 23 | Led flashes in an SOS cycle. If the button is pressed once, it flashes one round of SOS. If the button remains pressed, the LED flashes for as long as the button remains pressed

<img src="images/1.jpg" alt="Setup" width="700">

### Serial Output from each mode:
```bash
$ cargo run
I (30) boot: ESP-IDF v5.1-beta1-378-gea5e0ff298-dirt 2nd stage bootloader
I (30) boot: compile time Jun  7 2023 07:48:23
I (33) boot: Multicore bootloader
I (37) boot: chip revision: v1.0
I (40) boot.esp32: SPI Speed      : 40MHz
I (45) boot.esp32: SPI Mode       : DIO
I (50) boot.esp32: SPI Flash Size : 4MB
I (54) boot: Enabling RNG early entropy source...
I (60) boot: Partition Table:
I (63) boot: ## Label            Usage          Type ST Offset   Length
I (70) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (78) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (85) boot:  2 factory          factory app      00 00 00010000 003f0000
I (93) boot: End of partition table
I (97) esp_image: segment 0: paddr=00010020 vaddr=3f400020 size=02b90h ( 11152) map
I (110) esp_image: segment 1: paddr=00012bb8 vaddr=3ffb0000 size=0080ch (  2060) load
I (115) esp_image: segment 2: paddr=000133cc vaddr=40080000 size=01090h (  4240) load
I (124) esp_image: segment 3: paddr=00014464 vaddr=00000000 size=0bbb4h ( 48052)
I (148) esp_image: segment 4: paddr=00020020 vaddr=400d0020 size=04860h ( 18528) map
I (156) boot: Loaded app from partition at offset 0x10000
I (156) boot: Disabling RNG early entropy source...
Led Mode: Off                # -> nothing is pressed
Led Mode: On                 # -> button 1 is pressed
Led Mode: Off                # -> button isn't pressed any more
Led Mode: Blinky             # -> butto 2 is pressed 
Led Mode: Off                # -> button isn't pressed any more
Led Mode: SOS                # -> button 3 is pressed
Led Mode: Off                # -> button isn't pressed any more
```



### Info:
[The Rust Programming Language](https://doc.rust-lang.org/stable/book/)<br>
[Embedded Rust(no_std) on Espressif](https://docs.esp-rs.org/no_std-training/01_intro.html)<br>
[esp-hal](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-beta.1/esp32/esp_hal/index.html)<br>
[esp_println](https://docs.espressif.com/projects/rust/esp-println/0.14.0/esp_println/)<br>

