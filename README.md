# Embedded-Rust-on-esp32-different-Blink-Mode

This is my first little Rust program for the esp32. 

It consists of an ESP32, 3 buttons and on LED. Each button has it's own mode:

Button 1 (white): Led lights up as long as the button is pressed

Button 2 (red): Led flashes with an delay of 500 milliseconds as long as the button is pressed

Button 3 (blue9: Led flashes in an SOS cycle. If the button is pressed once, it flashes one round of SOS. If the button remains pressed, the LED flashes for as long as the button remains pressed

![Setup](images/1.jpg)


