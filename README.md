Transmits alarm panel sensor states (reed switches) to mqtt.

Sensor wires are connected to an STM32 "blue pill" chip, which reads the state of each
pin defined in `main.rs` and communicates changes over a serial connection.

An ESP8266 reads the changes by connecting to the stm32 serial pins, and pushes the changes
to an mqtt server.
