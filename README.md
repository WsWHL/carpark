## STM32 CARPARK

### Module devices:

- MCU: STM32F103C8T6
- OLED: OLED 0.96(I2C)
- RFID: RFID-RC522(SPI)
- VOICE: LU6288(MR628)(Serial)
- SG: SG90(PWM)
- DHT: DHT11

### Debuger:

ST-LINK V2 protocol SWD

#### dev command

1. startup openocd

   ```shell
   openocd
   ```

2. debug

    config: `.gdbinit`
    ```shell
    arm-none-eabi-gdb -se /Users/wanghailin/Works/stm32/carpark/target/thumbv7m-none-eabi/release/carpark
    ```

3. flash

    ```shell
    cargo flash --chip STM32F103C8 --protocol swd --reset-halt --release
    ```
