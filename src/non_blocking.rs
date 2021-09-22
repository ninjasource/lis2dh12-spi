use crate::*;
use accelerometer::vector::I16x3;
use embassy_traits::spi::FullDuplex;
use embedded_hal::digital::v2::OutputPin;

pub use accelerometer::{vector::F32x3, Accelerometer};

impl<SPI, SpiError, CS, PinError> Lis2dh12<SPI, CS>
where
    SPI: FullDuplex<u8, Error = SpiError>,
    CS: OutputPin<Error = PinError>,
{
    pub async fn check_who_am_i(&mut self) -> Result<(), Error<SpiError, PinError>> {
        self.cs.set_high().map_err(Error::Pin)?;
        let device_id = self.get_device_id().await?;
        if device_id != reg::DEVICE_ID {
            return Err(Error::InvalidWhoAmI(device_id));
        }
        Ok(())
    }

    pub async fn get_device_id(&mut self) -> Result<u8, Error<SpiError, PinError>> {
        self.read_reg(Register::WHO_AM_I).await
    }

    /// Block data update,
    /// `CTRL_REG4`: `BDU`
    pub async fn set_bdu(&mut self, bdu: bool) -> Result<(), Error<SpiError, PinError>> {
        self.reg_xset_bits(Register::CTRL_REG4, BDU, bdu).await?;
        Ok(())
    }

    /// Temperature sensor enable,
    /// `TEMP_CFG_REG`: `TEMP_EN`,
    /// the `BDU` bit in `CTRL_REG4` is also set
    pub async fn enable_temp(&mut self, enable: bool) -> Result<(), Error<SpiError, PinError>> {
        self.reg_xset_bits(Register::TEMP_CFG_REG, TEMP_EN, enable)
            .await?;
        if enable {
            // enable block data update (required for temp reading)
            self.reg_set_bits(Register::CTRL_REG4, BDU).await?;
        }
        Ok(())
    }

    #[inline]
    async fn reg_xset_bits(
        &mut self,
        reg: Register,
        bits: u8,
        set: bool,
    ) -> Result<(), Error<SpiError, PinError>> {
        if set {
            self.reg_set_bits(reg, bits).await
        } else {
            self.reg_reset_bits(reg, bits).await
        }
    }

    pub async fn set_output_data_rate(
        &mut self,
        odr: OutputDataRate,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(Register::CTRL_REG1, |v| {
            (v & !ODR_MASK) | ((odr as u8) << 4)
        })
        .await?;
        // By design, when the device from high-resolution configuration (HR) is set to power-down mode (PD),
        // it is recommended to read register REFERENCE (26h) for a complete reset of the filtering block
        // before switching to normal/high-performance mode again.
        if let OutputDataRate::PowerDown = odr {
            self.get_ref().await?;
        }
        Ok(())
    }

    /// `REFERENCE` register
    pub async fn get_ref(&mut self) -> Result<u8, Error<SpiError, PinError>> {
        self.read_reg(Register::REFERENCE).await
    }

    /*
        pub async fn set_low_power_mode(
            &mut self,
            low_power_mode: LowPowerMode,
        ) -> Result<(), Error<SpiError, PinError>> {
            let reset_bits = 0b0000_0011;
            self.reg_reset_bits(Register::CTRL1, reset_bits).await?;
            self.reg_set_bits(Register::CTRL1, low_power_mode as u8)
                .await?;
            Ok(())
        }

        pub async fn set_operating_mode(
            &mut self,
            mode: OperatingMode,
        ) -> Result<(), Error<SpiError, PinError>> {
            let reset_bits = 0b0000_1100;
            let set_bits = (mode as u8) << 2;
            self.reg_reset_bits(Register::CTRL1, reset_bits).await?;
            self.reg_set_bits(Register::CTRL1, set_bits).await?;
            Ok(())
        }

        pub async fn set_low_noise(
            &mut self,
            is_enabled: bool,
        ) -> Result<(), Error<SpiError, PinError>> {
            let bits = 0b0000_0100;
            if is_enabled {
                self.reg_set_bits(Register::CTRL1, bits).await?;
            } else {
                self.reg_reset_bits(Register::CTRL1, bits).await?;
            }

            Ok(())
        }

        pub async fn set_full_scale_selection(
            &mut self,
            full_scale_selection: FullScaleSelection,
        ) -> Result<(), Error<SpiError, PinError>> {
            let reset_bits = 0b0011_0000;
            let set_bits = (full_scale_selection as u8) << 4;
            self.reg_reset_bits(Register::CTRL6, reset_bits).await?;
            self.reg_set_bits(Register::CTRL6, set_bits).await?;

            #[cfg(feature = "out_f32")]
            {
                self.scale = full_scale_selection;
            }

            Ok(())
        }

        pub async fn set_output_data_rate(
            &mut self,
            odr: OutputDataRate,
        ) -> Result<(), Error<SpiError, PinError>> {
            let reset_bits = 0b1111_0000;
            let set_bits = (odr as u8) << 4;
            self.reg_reset_bits(Register::CTRL1, reset_bits).await?;
            self.reg_set_bits(Register::CTRL1, set_bits).await?;
            Ok(())
        }



        /// Temperature sensor data,
        /// `OUT_T_H`, `OUT_T_L`
        pub async fn get_temperature_raw(&mut self) -> Result<(i8, u8), Error<SpiError, PinError>> {
            let mut buf = [0u8; 2];
            self.read_regs(Register::OUT_T_L, &mut buf).await?;
            Ok((buf[1] as i8, buf[0]))
        }

        /// Temperature sensor data as float, only to be called in high power mode
        /// `OUT_T_H`, `OUT_T_L` converted to `f32`
        #[cfg(feature = "out_f32")]
        pub async fn get_temperature_high_power(&mut self) -> Result<f32, Error<SpiError, PinError>> {
            let (out_h, out_l) = self.get_temperature_raw().await?;

            // 12-bit resolution
            let value = (((out_h as i16) << 4) | ((out_l >> 4) as i16)) as i16; // 12 bit mode
            Ok(value as f32 * 0.0625 + 25.0) // in 12 bit mode each value is 16th of a degree C. Midpoint 25C
        }

        pub async fn get_temperature_low_power(&mut self) -> Result<i8, Error<SpiError, PinError>> {
            Ok(self.read_reg(Register::OUT_T).await? as i8 + 25) // midpoint is 25C
        }

        pub async fn get_raw(&mut self) -> Result<I16x3, Error<SpiError, PinError>> {
            let mut buf = [0u8; 6];
            self.read_regs(Register::OUT_X_L, &mut buf).await?;

            Ok(I16x3::new(
                ((buf[0] as u16) + ((buf[1] as u16) << 8)) as i16,
                ((buf[2] as u16) + ((buf[3] as u16) << 8)) as i16,
                ((buf[4] as u16) + ((buf[5] as u16) << 8)) as i16,
            ))
        }

        /// Get normalized ±g reading from the accelerometer
        #[cfg(feature = "out_f32")]
        pub async fn get_norm(&mut self) -> Result<F32x3, Error<SpiError, PinError>> {
            let acc_raw: I16x3 = self.get_raw().await?;

            let sensitivity: f32 = match self.scale {
                FullScaleSelection::PlusMinus2G => 0.000061037, // 1 / (MAX(i16) / 2)
                FullScaleSelection::PlusMinus4G => 0.000122074, // 1 / (MAX(i16) / 4)
                FullScaleSelection::PlusMinus8G => 0.000244148, // 1 / (MAX(i16) / 8)
                FullScaleSelection::PlusMinus16G => 0.000488296, // 1 / (MAX(i16) / 16)
            };

            Ok(F32x3::new(
                acc_raw.x as f32 * sensitivity,
                acc_raw.y as f32 * sensitivity,
                acc_raw.z as f32 * sensitivity,
            ))
        }

    */

    /// Operating mode selection,
    /// `CTRL_REG1`: `LPen` bit,
    /// `CTRL_REG4`: `HR` bit
    pub async fn set_operating_mode(
        &mut self,
        mode: OperatingMode,
    ) -> Result<(), Error<SpiError, PinError>> {
        match mode {
            OperatingMode::LowPower => {
                self.reg_reset_bits(Register::CTRL_REG4, HR).await?;
                self.reg_set_bits(Register::CTRL_REG1, LPen).await?;
            }
            OperatingMode::Normal => {
                self.reg_reset_bits(Register::CTRL_REG1, LPen).await?;
                self.reg_reset_bits(Register::CTRL_REG4, HR).await?;
            }
            OperatingMode::HighResolution => {
                self.reg_reset_bits(Register::CTRL_REG1, LPen).await?;
                self.reg_set_bits(Register::CTRL_REG4, HR).await?;
            }
        }
        Ok(())
    }

    pub async fn get_raw(&mut self) -> Result<I16x3, Error<SpiError, PinError>> {
        let mut buf = [0u8; 6];
        self.read_regs(Register::OUT_X_L, &mut buf).await?;

        Ok(I16x3::new(
            ((buf[0] as u16) + ((buf[1] as u16) << 8)) as i16,
            ((buf[2] as u16) + ((buf[3] as u16) << 8)) as i16,
            ((buf[4] as u16) + ((buf[5] as u16) << 8)) as i16,
        ))
    }

    /// Full-scale selection,
    /// `CTRL_REG4`: `FS`
    pub async fn set_scale(
        &mut self,
        fs: FullScaleSelection,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(Register::CTRL_REG4, |v| (v & !FS_MASK) | ((fs as u8) << 4))
            .await?;
        self.scale = fs;
        Ok(())
    }

    /// X,Y,Z-axis enable,
    /// `CTRL_REG1`: `Xen`, `Yen`, `Zen`
    pub async fn enable_axis(
        &mut self,
        (x, y, z): (bool, bool, bool),
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(Register::CTRL_REG1, |mut v| {
            v &= !(Xen | Yen | Zen); // disable all axises
            v |= if x { Xen } else { 0 };
            v |= if y { Yen } else { 0 };
            v |= if z { Zen } else { 0 };
            v
        })
        .await?;
        Ok(())
    }

    /// Get normalized ±g reading from the accelerometer
    pub async fn get_norm(&mut self) -> Result<F32x3, Error<SpiError, PinError>> {
        let acc_raw: I16x3 = self.get_raw().await?;

        let sensitivity: f32 = match self.scale {
            FullScaleSelection::PlusMinus2G => 0.001,
            FullScaleSelection::PlusMinus4G => 0.002,
            FullScaleSelection::PlusMinus8G => 0.004,
            FullScaleSelection::PlusMinus16G => 0.012,
        };

        Ok(F32x3::new(
            (acc_raw.x >> 4) as f32 * sensitivity, // 12-bit data
            (acc_raw.y >> 4) as f32 * sensitivity,
            (acc_raw.z >> 4) as f32 * sensitivity,
        ))

        /*
        let acc_raw: I16x3 = self.get_raw().await?;

        let sensitivity: f32 = match self.scale {
            FullScaleSelection::PlusMinus2G => 0.000061037, // 1 / (MAX(i16) / 2)
            FullScaleSelection::PlusMinus4G => 0.000122074, // 1 / (MAX(i16) / 4)
            FullScaleSelection::PlusMinus8G => 0.000244148, // 1 / (MAX(i16) / 8)
            FullScaleSelection::PlusMinus16G => 0.000488296, // 1 / (MAX(i16) / 16)
        };

        Ok(F32x3::new(
            acc_raw.x as f32 * sensitivity,
            acc_raw.y as f32 * sensitivity,
            acc_raw.z as f32 * sensitivity,
        ))*/
    }

    /// Data status,
    /// `STATUS_REG`: as
    /// DataStatus {zyxor: `ZYXOR`, xyzor: (`XOR`, `YOR`, `ZOR`), zyxda: `ZYXDA`, xyzda: (`XDA`, `YDA`, `ZDA`)}
    pub async fn get_status(&mut self) -> Result<DataStatus, Error<SpiError, PinError>> {
        let reg = self.read_reg(Register::STATUS_REG).await?;
        Ok(DataStatus {
            zyxor: (reg & ZYXOR) != 0,
            xyzor: ((reg & XOR) != 0, (reg & YOR) != 0, (reg & ZOR) != 0),
            zyxda: (reg & ZYXDA) != 0,
            xyzda: ((reg & XDA) != 0, (reg & YDA) != 0, (reg & ZDA) != 0),
        })
    }

    /// Temperature sensor data,
    /// `OUT_TEMP_H`, `OUT_TEMP_L`
    pub async fn get_temperature_raw(&mut self) -> Result<(i8, u8), Error<SpiError, PinError>> {
        let mut buf = [0u8; 2];
        self.read_regs(Register::OUT_TEMP_L, &mut buf).await?;
        Ok((buf[1] as i8, buf[0]))
    }

    /// Temperature sensor data,
    /// `OUT_TEMP_H`, `OUT_TEMP_L`
    pub async fn get_temperature_raw_foo(&mut self) -> Result<(u8, u8), Error<SpiError, PinError>> {
        let mut buf = [0u8; 2];
        self.read_regs(Register::OUT_TEMP_L, &mut buf).await?;
        Ok((buf[1], buf[0]))
    }

    /// Temperature sensor data as float,
    /// `OUT_TEMP_H`, `OUT_TEMP_L` converted to `f32`
    pub async fn get_temperature_c(&mut self) -> Result<f32, Error<SpiError, PinError>> {
        let (out_h, out_l) = self.get_temperature_raw().await?;

        // 10-bit resolution left aligned
        // out_h is twos compliment so: u8 -> i8 (change) -> i16 (no change)
        // out_h    | out_l
        // hhhhhhhh | ll000000
        // out_h: hhhhhhhh         -> 00000000hhhhhhhh -> hhhhhhhh00000000
        // out_l: ll000000         -> 00000000ll000000
        // value: hhhhhhhhll000000 -> 000000xxxxxxxxll
        let value = (((out_h as i16) << 8) | (out_l as i16)) >> 6;

        // mid point is 25 degrees
        // 10bit signal is divided by 4
        Ok((value as f32 * 0.25) + 25.0)
    }

    /// Enable interrupt double-click on X,Y,Z axis,
    /// `CLICK_CFG`: `XD`, `YD`, `ZD`
    pub async fn enable_double_click(
        &mut self,
        (x, y, z): (bool, bool, bool),
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(Register::CLICK_CFG, |mut v| {
            v &= !(XD | YD | ZD); // disable all axises
            v |= if x { XD } else { 0 };
            v |= if y { YD } else { 0 };
            v |= if z { ZD } else { 0 };
            v
        })
        .await?;
        Ok(())
    }

    /// AOI-6D Interrupt mode,
    /// `INTx_CFG`: `AOI`, `6D`
    pub async fn int1_set_mode(&mut self, mode: Aoi6d) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(Register::INT1_CFG, |v| {
            (v & !AOI_6D_MASK) | ((mode as u8) << 6)
        })
        .await?;
        Ok(())
    }

    /// AOI-6D Interrupt mode,
    /// `INTx_CFG`: `AOI`, `6D`
    pub async fn int1_enable_click(
        &mut self,
        enable: bool,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.reg_xset_bits(Register::INT1_CFG, INT1_CFG_ENABLE_CLICK, enable)
            .await?;
        Ok(())
    }

    /// `CLICK` interrupt on `INT1` pin,
    /// `CTRL_REG3`: `I1_CLICK`
    pub async fn enable_i1_click(&mut self, enable: bool) -> Result<(), Error<SpiError, PinError>> {
        self.reg_xset_bits(Register::CTRL_REG3, I1_CLICK, enable)
            .await?;
        Ok(())
    }

    /// Latch interrupt request on INT1_SRC (31h),
    /// with INT1_SRC (31h) register cleared by reading INT1_SRC (31h) itself,
    /// `CTRL_REG5`: `LIR_INT1`
    pub async fn enable_lir_int1(&mut self, latch: bool) -> Result<(), Error<SpiError, PinError>> {
        self.reg_xset_bits(Register::CTRL_REG5, LIR_INT1, latch)
            .await?;
        Ok(())
    }

    /// Enable interrupt single-click on X,Y,Z axis,
    /// `CLICK_CFG`: `XS`, `YS`, `ZS`
    pub async fn enable_single_click(
        &mut self,
        (x, y, z): (bool, bool, bool),
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(Register::CLICK_CFG, |mut v| {
            v &= !(XS | YS | ZS); // disable all axises
            v |= if x { XS } else { 0 };
            v |= if y { YS } else { 0 };
            v |= if z { ZS } else { 0 };
            v
        })
        .await?;
        Ok(())
    }

    /// INT1/INT2 pin polarity,
    /// `CTRL_REG6`: `INT_POLARITY`
    pub async fn set_int_polarity(
        &mut self,
        active_low: bool,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.reg_xset_bits(Register::CTRL_REG6, INT_POLARITY, active_low)
            .await?;
        Ok(())
    }

    /// Click threshold,
    /// `CLICK_THS`: `Ths`
    pub async fn set_click_ths(&mut self, ths: u8) -> Result<(), Error<SpiError, PinError>> {
        self.write_reg(Register::CLICK_THS, ths & THS_MASK).await?;
        Ok(())
    }

    async fn reg_set_bits(
        &mut self,
        reg: Register,
        bits: u8,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(reg, |v| v | bits).await
    }

    async fn reg_reset_bits(
        &mut self,
        reg: Register,
        bits: u8,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.modify_reg(reg, |v| v & !bits).await
    }

    async fn modify_reg<F>(&mut self, reg: Register, f: F) -> Result<(), Error<SpiError, PinError>>
    where
        F: FnOnce(u8) -> u8,
    {
        let r = self.read_reg(reg).await?;
        self.write_reg(reg, f(r)).await?;
        Ok(())
    }

    async fn write_reg(
        &mut self,
        register: Register,
        data: u8,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.chip_select()?;
        let result = self.write_then_write(register.addr(), data).await;
        self.chip_deselect()?;
        result
    }

    async fn read_reg(&mut self, register: Register) -> Result<u8, Error<SpiError, PinError>> {
        self.chip_select()?;
        let request = 0b1000_0000 | register.addr(); // set the read bit
        let result = self.write_then_read(request).await;
        self.chip_deselect()?;
        result
    }

    async fn read_regs(
        &mut self,
        register: Register,
        buf: &mut [u8],
    ) -> Result<(), Error<SpiError, PinError>> {
        self.chip_select()?;
        let request = 0b1100_0000 | register.addr(); // set the read bit and register increment bit
        let result = self.write_then_read_into(request, buf).await;
        self.chip_deselect()?;
        result
    }

    async fn write_then_read(&mut self, request: u8) -> Result<u8, Error<SpiError, PinError>> {
        self.spi.write(&[request]).await?;
        let mut data = [0; 1];
        self.spi.read(&mut data).await?;
        Ok(data[0])
    }

    async fn write_then_read_into(
        &mut self,
        request: u8,
        buf: &mut [u8],
    ) -> Result<(), Error<SpiError, PinError>> {
        self.spi.write(&[request]).await?;

        let mut data = [0; 1];
        for x in buf {
            self.spi.read(&mut data).await?;
            *x = data[0];
        }

        Ok(())
    }

    async fn write_then_write(
        &mut self,
        request: u8,
        data: u8,
    ) -> Result<(), Error<SpiError, PinError>> {
        self.spi.write(&[request]).await?;
        self.spi.write(&[data]).await?;
        Ok(())
    }

    fn chip_select(&mut self) -> Result<(), Error<SpiError, PinError>> {
        self.cs.set_low().map_err(Error::Pin)
    }

    fn chip_deselect(&mut self) -> Result<(), Error<SpiError, PinError>> {
        self.cs.set_high().map_err(Error::Pin)
    }
}
