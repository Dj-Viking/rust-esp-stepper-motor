# Dependencies

* rust and cargo
* espflash 
	- to use this you need to `cargo install espflash` so that it's available in your shell path
	- allows you to flash the esp32 with the binary target that cargo compiled from the rust code and create packages
---

# Architecture

```mermaid
flowchart TD

    Dev[Developer]

    Dev --> BuildScript[build.sh]
    BuildScript --> CargoBuild[cargo build]
    CargoBuild --> BuildRs[build.rs]
    BuildRs --> Linker[linkall.x linker configuration]

    CargoBuild --> Firmware[Firmware Binary]

    Firmware --> ESP[ESP32 MCU]

    subgraph ESP32 Firmware
        Main[main.rs]

        Main --> PanicHandler[USB Serial JTAG Panic Handler]

        Main --> GPIOConfig[GPIO Configuration]

        GPIOConfig --> EnablePin[ENABLE GPIO5]
        GPIOConfig --> MicrostepPins[M0 GPIO6<br/>M1 GPIO7<br/>M2 GPIO8]
        GPIOConfig --> StepPin[STEP GPIO0]
        GPIOConfig --> DirectionPin[DIR GPIO1]
        GPIOConfig --> SleepPin[SLP GPIO4]
        GPIOConfig --> ResetPin[RST GPIO9]

        Main --> Delay[esp-hal Delay]

        Main --> Loop[Main Loop]

        Loop --> StepFunction[step-on-delay]
        StepFunction --> StepHigh[STEP HIGH]
        StepFunction --> Delay10A[10ms Delay]
        StepFunction --> StepLow[STEP LOW]
        StepFunction --> Delay10B[10ms Delay]
        Delay10B --> StepFunction
    end

    StepPin --> DRV8825[DRV8825 Driver]
    DirectionPin --> DRV8825
    EnablePin --> DRV8825
    MicrostepPins --> DRV8825
    SleepPin --> DRV8825
    ResetPin --> DRV8825

    DRV8825 --> Motor[Stepper Motor]
```

