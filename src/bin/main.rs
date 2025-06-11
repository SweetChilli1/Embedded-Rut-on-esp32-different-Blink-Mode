#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock, 
    delay::{ Delay}, 
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull}, 
    main, 
};
use esp_println::println;
// You need a panic handler. Usually, you you would use esp_backtrace, panic-probe, or
// something similar, but you can also bring your own like this:
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[main]
fn main() -> ! {

    //this part defines the config and each pin:
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config); 
    let input_config = InputConfig::default().with_pull(Pull::Down);

    //defines the button
    let button1 = Input::new(peripherals.GPIO4, input_config);
    let button2 = Input::new(peripherals.GPIO21, input_config);
    let button3 = Input::new(peripherals.GPIO23, input_config);

    //defines the led
    let mut led = Output::new(peripherals.GPIO13, Level::High, OutputConfig::default());
    
    //generate delay Instance
    let mut delay = Delay::new();
  
    /* as long the esp32 is powered on, this loop runs over and over, this ist the interactiv parth. 
     In this loop the 'if' conditions ckecks what button is pressed and execut , the right pards*/
    loop {                                        
        
        //if the 'if' condition ist true, it goes in a loop and axecutes the defined mode, as long as the button is pressed. 
        if check_truth(&button1) {                
            println!("Led Mode: On");
            while !break_loop(&button1) {
                LedMode::On.run(&mut led, &mut delay);
                }
        } else if check_truth(&button2) {
            println!("Led Mode: Blinky");
            while !break_loop(&button2) {
                LedMode::Blinky.run(&mut led, &mut delay);
                }
        } else if check_truth(&button3) {
            println!("Led Mode: SOS");
            while !break_loop(&button3) {
                LedMode::Sos.run(&mut led, &mut delay);
                }
        } else {
            println!("Led Mode: Off");
            while button1.is_low() && button2.is_low() && button3.is_low() {
                LedMode::Off.run(&mut led, &mut delay);
                }
            }
        }
}

//is the half of the SoS blink, for a SOS you need two with other delay
fn half_sos(led: &mut Output<'_>,delay: &Delay, millis: u32) { 
    for _ in 0..3 {  // for '_' = <no counter> in range from 1 to 3 
        led.set_high();
        delay.delay_millis(millis);

        led.set_low();
        delay.delay_millis(200); // 200 is the default value for led.set_low() 
    }
}

//check if button <xyz> is pressed and returns a bool => true
fn check_truth(button: &Input<'_>) -> bool {  
    button.is_high()
}

 //ckeck if isn't pressed anymore, then returns true, to break the loop
fn break_loop(button: &Input<'_>) -> bool { 
    button.is_low()
}
//define the different modes 
enum LedMode {
    On,
    Blinky,
    Sos,
    Off,
}

//different the funktion for each mode
impl LedMode {
    fn run(&self, led: &mut Output<'_>, delay: &mut Delay) {
        match self {
            LedMode::Off => {
                led.set_low();
            } 
            LedMode::On => {
                led.set_high();
            }
            LedMode::Blinky => {
                led.set_high();
                delay.delay_millis(500);
                led.set_low();
                delay.delay_millis(500); 
            }
            LedMode::Sos => {
                half_sos(led, delay, 200);
                half_sos(led, delay, 700);
            }

        }
    }
}
