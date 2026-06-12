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

# Component View

```text
+--------------------------------------------------+
|                  Host Computer                   |
|--------------------------------------------------|
| build.sh                                         |
| cargo build                                      |
| espflash                                         |
+----------------------+---------------------------+
                       |
                       v
+--------------------------------------------------+
|                    build.rs                      |
|--------------------------------------------------|
| linker configuration                             |
| linker diagnostics                               |
| missing symbol troubleshooting                   |
+----------------------+---------------------------+
                       |
                       v
+--------------------------------------------------+
|                ESP32 Firmware                    |
|--------------------------------------------------|
| main.rs                                          |
| panic handler                                    |
| GPIO initialization                              |
| motor control loop                               |
+----------------------+---------------------------+
                       |
                       v
+--------------------------------------------------+
|                Motor Control Layer               |
|--------------------------------------------------|
| step_on_delay()                                  |
| pulse generation                                 |
| fixed timing (10ms high / 10ms low)              |
+----------------------+---------------------------+
                       |
                       v
+--------------------------------------------------+
|                Hardware Interface                |
|--------------------------------------------------|
| GPIO0  -> STEP                                   |
| GPIO1  -> DIR                                    |
| GPIO4  -> SLEEP                                  |
| GPIO5  -> ENABLE                                 |
| GPIO6  -> M0                                     |
| GPIO7  -> M1                                     |
| GPIO8  -> M2                                     |
| GPIO9  -> RESET                                  |
+----------------------+---------------------------+
                       |
                       v
+--------------------------------------------------+
|                   DRV8825                        |
+----------------------+---------------------------+
                       |
                       v
+--------------------------------------------------+
|                 Stepper Motor                    |
+--------------------------------------------------+
```
