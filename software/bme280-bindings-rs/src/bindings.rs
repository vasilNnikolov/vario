/* automatically generated by rust-bindgen 0.71.1 */

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub const INT8_MAX: u8 = 127;
pub const INT8_MIN: i8 = -128;
pub const UINT8_MAX: u8 = 255;
pub const INT16_MAX: u16 = 32767;
pub const INT16_MIN: i16 = -32768;
pub const UINT16_MAX: u16 = 65535;
pub const INT32_MAX: u32 = 2147483647;
pub const INT32_MIN: i32 = -2147483648;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u8 = 127;
pub const INT_LEAST8_MIN: i8 = -128;
pub const UINT_LEAST8_MAX: u8 = 255;
pub const INT_LEAST16_MAX: u16 = 32767;
pub const INT_LEAST16_MIN: i16 = -32768;
pub const UINT_LEAST16_MAX: u16 = 65535;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u8 = 127;
pub const INT_FAST8_MIN: i8 = -128;
pub const UINT_FAST8_MAX: u8 = 255;
pub const INT_FAST16_MAX: u16 = 32767;
pub const INT_FAST16_MIN: i16 = -32768;
pub const UINT_FAST16_MAX: u16 = 65535;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u32 = 2147483647;
pub const INTPTR_MIN: i32 = -2147483648;
pub const UINTPTR_MAX: u32 = 4294967295;
pub const INTMAX_MAX: u64 = 9223372036854775807;
pub const INTMAX_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u32 = 2147483647;
pub const PTRDIFF_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: u32 = 4294967295;
pub const WCHAR_MAX: u32 = 4294967295;
pub const WINT_MAX: u32 = 2147483647;
pub const TRUE: u8 = 1;
pub const FALSE: u8 = 0;
pub const BME280_INTF_RET_SUCCESS: u8 = 0;
pub const BME280_OK: u8 = 0;
pub const BME280_E_NULL_PTR: i8 = -1;
pub const BME280_E_COMM_FAIL: i8 = -2;
pub const BME280_E_INVALID_LEN: i8 = -3;
pub const BME280_E_DEV_NOT_FOUND: i8 = -4;
pub const BME280_E_SLEEP_MODE_FAIL: i8 = -5;
pub const BME280_E_NVM_COPY_FAILED: i8 = -6;
pub const BME280_W_INVALID_OSR_MACRO: u8 = 1;
pub const BME280_CHIP_ID: u8 = 96;
pub const BME280_I2C_ADDR_PRIM: u8 = 118;
pub const BME280_I2C_ADDR_SEC: u8 = 119;
pub const BME280_REG_CHIP_ID: u8 = 208;
pub const BME280_REG_RESET: u8 = 224;
pub const BME280_REG_TEMP_PRESS_CALIB_DATA: u8 = 136;
pub const BME280_REG_HUMIDITY_CALIB_DATA: u8 = 225;
pub const BME280_REG_CTRL_HUM: u8 = 242;
pub const BME280_REG_STATUS: u8 = 243;
pub const BME280_REG_PWR_CTRL: u8 = 244;
pub const BME280_REG_CTRL_MEAS: u8 = 244;
pub const BME280_REG_CONFIG: u8 = 245;
pub const BME280_REG_DATA: u8 = 247;
pub const BME280_LEN_TEMP_PRESS_CALIB_DATA: u8 = 26;
pub const BME280_LEN_HUMIDITY_CALIB_DATA: u8 = 7;
pub const BME280_LEN_P_T_H_DATA: u8 = 8;
pub const BME280_POWERMODE_SLEEP: u8 = 0;
pub const BME280_POWERMODE_FORCED: u8 = 1;
pub const BME280_POWERMODE_NORMAL: u8 = 3;
pub const BME280_SENSOR_MODE_MSK: u8 = 3;
pub const BME280_SENSOR_MODE_POS: u8 = 0;
pub const BME280_SOFT_RESET_COMMAND: u8 = 182;
pub const BME280_STATUS_IM_UPDATE: u8 = 1;
pub const BME280_STATUS_MEAS_DONE: u8 = 8;
pub const BME280_PRESS: u8 = 1;
pub const BME280_TEMP: u8 = 2;
pub const BME280_HUM: u8 = 4;
pub const BME280_ALL: u8 = 7;
pub const BME280_SEL_OSR_PRESS: u8 = 1;
pub const BME280_SEL_OSR_TEMP: u8 = 2;
pub const BME280_SEL_OSR_HUM: u8 = 4;
pub const BME280_SEL_FILTER: u8 = 8;
pub const BME280_SEL_STANDBY: u8 = 16;
pub const BME280_SEL_ALL_SETTINGS: u8 = 31;
pub const BME280_NO_OVERSAMPLING: u8 = 0;
pub const BME280_OVERSAMPLING_1X: u8 = 1;
pub const BME280_OVERSAMPLING_2X: u8 = 2;
pub const BME280_OVERSAMPLING_4X: u8 = 3;
pub const BME280_OVERSAMPLING_8X: u8 = 4;
pub const BME280_OVERSAMPLING_16X: u8 = 5;
pub const BME280_OVERSAMPLING_MAX: u8 = 16;
pub const BME280_CTRL_HUM_MSK: u8 = 7;
pub const BME280_CTRL_HUM_POS: u8 = 0;
pub const BME280_CTRL_PRESS_MSK: u8 = 28;
pub const BME280_CTRL_PRESS_POS: u8 = 2;
pub const BME280_CTRL_TEMP_MSK: u8 = 224;
pub const BME280_CTRL_TEMP_POS: u8 = 5;
pub const BME280_MEAS_OFFSET: u16 = 1250;
pub const BME280_MEAS_DUR: u16 = 2300;
pub const BME280_PRES_HUM_MEAS_OFFSET: u16 = 575;
pub const BME280_MEAS_SCALING_FACTOR: u16 = 1000;
pub const BME280_STARTUP_DELAY: u16 = 2000;
pub const BME280_MAX_LEN: u8 = 10;
pub const BME280_STANDBY_TIME_0_5_MS: u8 = 0;
pub const BME280_STANDBY_TIME_62_5_MS: u8 = 1;
pub const BME280_STANDBY_TIME_125_MS: u8 = 2;
pub const BME280_STANDBY_TIME_250_MS: u8 = 3;
pub const BME280_STANDBY_TIME_500_MS: u8 = 4;
pub const BME280_STANDBY_TIME_1000_MS: u8 = 5;
pub const BME280_STANDBY_TIME_10_MS: u8 = 6;
pub const BME280_STANDBY_TIME_20_MS: u8 = 7;
pub const BME280_STANDBY_MSK: u8 = 224;
pub const BME280_STANDBY_POS: u8 = 5;
pub const BME280_12_BIT_SHIFT: u8 = 12;
pub const BME280_8_BIT_SHIFT: u8 = 8;
pub const BME280_4_BIT_SHIFT: u8 = 4;
pub const BME280_FILTER_COEFF_OFF: u8 = 0;
pub const BME280_FILTER_COEFF_2: u8 = 1;
pub const BME280_FILTER_COEFF_4: u8 = 2;
pub const BME280_FILTER_COEFF_8: u8 = 3;
pub const BME280_FILTER_COEFF_16: u8 = 4;
pub const BME280_FILTER_MSK: u8 = 28;
pub const BME280_FILTER_POS: u8 = 2;
pub type int_least8_t = ::core::ffi::c_schar;
pub type int_least16_t = ::core::ffi::c_short;
pub type int_least32_t = ::core::ffi::c_int;
pub type int_least64_t = ::core::ffi::c_longlong;
pub type uint_least8_t = ::core::ffi::c_uchar;
pub type uint_least16_t = ::core::ffi::c_ushort;
pub type uint_least32_t = ::core::ffi::c_uint;
pub type uint_least64_t = ::core::ffi::c_ulonglong;
pub type int_fast8_t = ::core::ffi::c_schar;
pub type int_fast16_t = ::core::ffi::c_short;
pub type int_fast32_t = ::core::ffi::c_int;
pub type int_fast64_t = ::core::ffi::c_longlong;
pub type uint_fast8_t = ::core::ffi::c_uchar;
pub type uint_fast16_t = ::core::ffi::c_ushort;
pub type uint_fast32_t = ::core::ffi::c_uint;
pub type uint_fast64_t = ::core::ffi::c_ulonglong;
pub type intmax_t = ::core::ffi::c_longlong;
pub type uintmax_t = ::core::ffi::c_ulonglong;
pub type wchar_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct max_align_t {
    pub __max_align_ll: ::core::ffi::c_longlong,
    pub __max_align_ld: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of max_align_t"][::core::mem::size_of::<max_align_t>() - 16usize];
    ["Alignment of max_align_t"][::core::mem::align_of::<max_align_t>() - 8usize];
    ["Offset of field: max_align_t::__max_align_ll"]
        [::core::mem::offset_of!(max_align_t, __max_align_ll) - 0usize];
    ["Offset of field: max_align_t::__max_align_ld"]
        [::core::mem::offset_of!(max_align_t, __max_align_ld) - 8usize];
};
#[doc = " SPI interface"]
pub const bme280_intf_BME280_SPI_INTF: bme280_intf = 0;
#[doc = " I2C interface"]
pub const bme280_intf_BME280_I2C_INTF: bme280_intf = 1;
#[doc = " @brief Interface selection Enums"]
pub type bme280_intf = ::core::ffi::c_uint;
#[doc = " @brief Bus communication function pointer which should be mapped to\n the platform specific read functions of the user\n\n @param[in] reg_addr       : Register address from which data is read.\n @param[out] reg_data      : Pointer to data buffer where read data is stored.\n @param[in] len            : Number of bytes of data to be read.\n @param[in, out] intf_ptr  : Void pointer that can enable the linking of descriptors\n                             for interface related call backs.\n\n @retval   0 -> Success.\n @retval Non zero value -> Fail.\n"]
pub type bme280_read_fptr_t = ::core::option::Option<
    unsafe extern "C" fn(
        reg_addr: u8,
        reg_data: *mut u8,
        len: u32,
        intf_ptr: *mut ::core::ffi::c_void,
    ) -> i8,
>;
#[doc = " @brief Bus communication function pointer which should be mapped to\n the platform specific write functions of the user\n\n @param[in] reg_addr      : Register address to which the data is written.\n @param[in] reg_data      : Pointer to data buffer in which data to be written\n                            is stored.\n @param[in] len           : Number of bytes of data to be written.\n @param[in, out] intf_ptr : Void pointer that can enable the linking of descriptors\n                            for interface related call backs\n\n @retval   0   -> Success.\n @retval Non zero value -> Fail.\n"]
pub type bme280_write_fptr_t = ::core::option::Option<
    unsafe extern "C" fn(
        reg_addr: u8,
        reg_data: *const u8,
        len: u32,
        intf_ptr: *mut ::core::ffi::c_void,
    ) -> i8,
>;
#[doc = " @brief Delay function pointer which should be mapped to\n delay function of the user\n\n @param[in] period              : Delay in microseconds.\n @param[in, out] intf_ptr       : Void pointer that can enable the linking of descriptors\n                                  for interface related call backs\n"]
pub type bme280_delay_us_fptr_t =
    ::core::option::Option<unsafe extern "C" fn(period: u32, intf_ptr: *mut ::core::ffi::c_void)>;
#[doc = " @brief Calibration data"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bme280_calib_data {
    #[doc = " Calibration coefficient for the temperature sensor"]
    pub dig_t1: u16,
    #[doc = " Calibration coefficient for the temperature sensor"]
    pub dig_t2: i16,
    #[doc = " Calibration coefficient for the temperature sensor"]
    pub dig_t3: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p1: u16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p2: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p3: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p4: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p5: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p6: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p7: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p8: i16,
    #[doc = " Calibration coefficient for the pressure sensor"]
    pub dig_p9: i16,
    #[doc = " Calibration coefficient for the humidity sensor"]
    pub dig_h1: u8,
    #[doc = " Calibration coefficient for the humidity sensor"]
    pub dig_h2: i16,
    #[doc = " Calibration coefficient for the humidity sensor"]
    pub dig_h3: u8,
    #[doc = " Calibration coefficient for the humidity sensor"]
    pub dig_h4: i16,
    #[doc = " Calibration coefficient for the humidity sensor"]
    pub dig_h5: i16,
    #[doc = " Calibration coefficient for the humidity sensor"]
    pub dig_h6: i8,
    #[doc = " Variable to store the intermediate temperature coefficient"]
    pub t_fine: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bme280_calib_data"][::core::mem::size_of::<bme280_calib_data>() - 40usize];
    ["Alignment of bme280_calib_data"][::core::mem::align_of::<bme280_calib_data>() - 4usize];
    ["Offset of field: bme280_calib_data::dig_t1"]
        [::core::mem::offset_of!(bme280_calib_data, dig_t1) - 0usize];
    ["Offset of field: bme280_calib_data::dig_t2"]
        [::core::mem::offset_of!(bme280_calib_data, dig_t2) - 2usize];
    ["Offset of field: bme280_calib_data::dig_t3"]
        [::core::mem::offset_of!(bme280_calib_data, dig_t3) - 4usize];
    ["Offset of field: bme280_calib_data::dig_p1"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p1) - 6usize];
    ["Offset of field: bme280_calib_data::dig_p2"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p2) - 8usize];
    ["Offset of field: bme280_calib_data::dig_p3"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p3) - 10usize];
    ["Offset of field: bme280_calib_data::dig_p4"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p4) - 12usize];
    ["Offset of field: bme280_calib_data::dig_p5"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p5) - 14usize];
    ["Offset of field: bme280_calib_data::dig_p6"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p6) - 16usize];
    ["Offset of field: bme280_calib_data::dig_p7"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p7) - 18usize];
    ["Offset of field: bme280_calib_data::dig_p8"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p8) - 20usize];
    ["Offset of field: bme280_calib_data::dig_p9"]
        [::core::mem::offset_of!(bme280_calib_data, dig_p9) - 22usize];
    ["Offset of field: bme280_calib_data::dig_h1"]
        [::core::mem::offset_of!(bme280_calib_data, dig_h1) - 24usize];
    ["Offset of field: bme280_calib_data::dig_h2"]
        [::core::mem::offset_of!(bme280_calib_data, dig_h2) - 26usize];
    ["Offset of field: bme280_calib_data::dig_h3"]
        [::core::mem::offset_of!(bme280_calib_data, dig_h3) - 28usize];
    ["Offset of field: bme280_calib_data::dig_h4"]
        [::core::mem::offset_of!(bme280_calib_data, dig_h4) - 30usize];
    ["Offset of field: bme280_calib_data::dig_h5"]
        [::core::mem::offset_of!(bme280_calib_data, dig_h5) - 32usize];
    ["Offset of field: bme280_calib_data::dig_h6"]
        [::core::mem::offset_of!(bme280_calib_data, dig_h6) - 34usize];
    ["Offset of field: bme280_calib_data::t_fine"]
        [::core::mem::offset_of!(bme280_calib_data, t_fine) - 36usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bme280_data {
    #[doc = " Compensated pressure"]
    pub pressure: u32,
    #[doc = " Compensated temperature"]
    pub temperature: i32,
    #[doc = " Compensated humidity"]
    pub humidity: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bme280_data"][::core::mem::size_of::<bme280_data>() - 12usize];
    ["Alignment of bme280_data"][::core::mem::align_of::<bme280_data>() - 4usize];
    ["Offset of field: bme280_data::pressure"]
        [::core::mem::offset_of!(bme280_data, pressure) - 0usize];
    ["Offset of field: bme280_data::temperature"]
        [::core::mem::offset_of!(bme280_data, temperature) - 4usize];
    ["Offset of field: bme280_data::humidity"]
        [::core::mem::offset_of!(bme280_data, humidity) - 8usize];
};
#[doc = " @brief bme280 sensor structure which comprises of uncompensated temperature,\n pressure and humidity data"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bme280_uncomp_data {
    #[doc = " Un-compensated pressure"]
    pub pressure: u32,
    #[doc = " Un-compensated temperature"]
    pub temperature: u32,
    #[doc = " Un-compensated humidity"]
    pub humidity: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bme280_uncomp_data"][::core::mem::size_of::<bme280_uncomp_data>() - 12usize];
    ["Alignment of bme280_uncomp_data"][::core::mem::align_of::<bme280_uncomp_data>() - 4usize];
    ["Offset of field: bme280_uncomp_data::pressure"]
        [::core::mem::offset_of!(bme280_uncomp_data, pressure) - 0usize];
    ["Offset of field: bme280_uncomp_data::temperature"]
        [::core::mem::offset_of!(bme280_uncomp_data, temperature) - 4usize];
    ["Offset of field: bme280_uncomp_data::humidity"]
        [::core::mem::offset_of!(bme280_uncomp_data, humidity) - 8usize];
};
#[doc = " @brief bme280 sensor settings structure which comprises of mode,\n oversampling and filter settings."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bme280_settings {
    #[doc = " Pressure oversampling"]
    pub osr_p: u8,
    #[doc = " Temperature oversampling"]
    pub osr_t: u8,
    #[doc = " Humidity oversampling"]
    pub osr_h: u8,
    #[doc = " Filter coefficient"]
    pub filter: u8,
    #[doc = " Standby time"]
    pub standby_time: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bme280_settings"][::core::mem::size_of::<bme280_settings>() - 5usize];
    ["Alignment of bme280_settings"][::core::mem::align_of::<bme280_settings>() - 1usize];
    ["Offset of field: bme280_settings::osr_p"]
        [::core::mem::offset_of!(bme280_settings, osr_p) - 0usize];
    ["Offset of field: bme280_settings::osr_t"]
        [::core::mem::offset_of!(bme280_settings, osr_t) - 1usize];
    ["Offset of field: bme280_settings::osr_h"]
        [::core::mem::offset_of!(bme280_settings, osr_h) - 2usize];
    ["Offset of field: bme280_settings::filter"]
        [::core::mem::offset_of!(bme280_settings, filter) - 3usize];
    ["Offset of field: bme280_settings::standby_time"]
        [::core::mem::offset_of!(bme280_settings, standby_time) - 4usize];
};
#[doc = " @brief bme280 device structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bme280_dev {
    #[doc = " Chip Id"]
    pub chip_id: u8,
    #[doc = " Interface Selection\n For SPI, intf = BME280_SPI_INTF\n For I2C, intf = BME280_I2C_INTF"]
    pub intf: bme280_intf,
    #[doc = " The interface pointer is used to enable the user\n to link their interface descriptors for reference during the\n implementation of the read and write interfaces to the\n hardware."]
    pub intf_ptr: *mut ::core::ffi::c_void,
    #[doc = " Variable to store result of read/write function"]
    pub intf_rslt: i8,
    #[doc = " Read function pointer"]
    pub read: bme280_read_fptr_t,
    #[doc = " Write function pointer"]
    pub write: bme280_write_fptr_t,
    #[doc = " Delay function pointer"]
    pub delay_us: bme280_delay_us_fptr_t,
    #[doc = " Trim data"]
    pub calib_data: bme280_calib_data,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bme280_dev"][::core::mem::size_of::<bme280_dev>() - 68usize];
    ["Alignment of bme280_dev"][::core::mem::align_of::<bme280_dev>() - 4usize];
    ["Offset of field: bme280_dev::chip_id"][::core::mem::offset_of!(bme280_dev, chip_id) - 0usize];
    ["Offset of field: bme280_dev::intf"][::core::mem::offset_of!(bme280_dev, intf) - 4usize];
    ["Offset of field: bme280_dev::intf_ptr"]
        [::core::mem::offset_of!(bme280_dev, intf_ptr) - 8usize];
    ["Offset of field: bme280_dev::intf_rslt"]
        [::core::mem::offset_of!(bme280_dev, intf_rslt) - 12usize];
    ["Offset of field: bme280_dev::read"][::core::mem::offset_of!(bme280_dev, read) - 16usize];
    ["Offset of field: bme280_dev::write"][::core::mem::offset_of!(bme280_dev, write) - 20usize];
    ["Offset of field: bme280_dev::delay_us"]
        [::core::mem::offset_of!(bme280_dev, delay_us) - 24usize];
    ["Offset of field: bme280_dev::calib_data"]
        [::core::mem::offset_of!(bme280_dev, calib_data) - 28usize];
};
impl Default for bme280_dev {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiInit\n \\page bme280_api_bme280_init bme280_init\n \\code\n int8_t bme280_init(struct bme280_dev *dev);\n \\endcode\n @details This API reads the chip-id of the sensor which is the first step to\n verify the sensor and also calibrates the sensor\n As this API is the entry point, call this API before using other APIs.\n\n @param[in,out] dev : Structure instance of bme280_dev\n\n @return Result of API execution status.\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_init(dev: *mut bme280_dev) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiRegister\n \\page bme280_api_bme280_set_regs bme280_set_regs\n \\code\n int8_t bme280_set_regs(const uint8_t reg_addr, const uint8_t *reg_data, uint32_t len, struct bme280_dev *dev);\n \\endcode\n @details This API writes the given data to the register address of the sensor\n\n @param[in] reg_addr : Register addresses to where the data is to be written\n @param[in] reg_data : Pointer to data buffer which is to be written\n                       in the reg_addr of sensor.\n @param[in] len      : No of bytes of data to write\n @param[in,out] dev  : Structure instance of bme280_dev\n\n @return Result of API execution status.\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_set_regs(
        reg_addr: *mut u8,
        reg_data: *const u8,
        len: u32,
        dev: *mut bme280_dev,
    ) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiRegister\n \\page bme280_api_bme280_get_regs bme280_get_regs\n \\code\n int8_t bme280_get_regs(uint8_t reg_addr, uint8_t *reg_data, uint32_t len, struct bme280_dev *dev);\n \\endcode\n @details This API reads the data from the given register address of sensor.\n\n @param[in] reg_addr  : Register address from where the data to be read\n @param[out] reg_data : Pointer to data buffer to store the read data.\n @param[in] len       : No of bytes of data to be read.\n @param[in,out] dev   : Structure instance of bme280_dev.\n\n @return Result of API execution status.\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_get_regs(reg_addr: u8, reg_data: *mut u8, len: u32, dev: *mut bme280_dev) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorSettings\n \\page bme280_api_bme280_set_sensor_settings bme280_set_sensor_settings\n \\code\n int8_t bme280_set_sensor_settings(uint8_t desired_settings, const struct bme280_settings *settings, struct bme280_dev *dev);\n \\endcode\n @details This API sets the oversampling, filter and standby duration\n (normal mode) settings in the sensor.\n\n @param[in] desired_settings  : Variable used to select the settings which\n                                are to be set in the sensor.\n @param[in] settings          : Structure instance of bme280_settings.\n @param[in] dev               : Structure instance of bme280_dev.\n\n @note : Below are the macros to be used by the user for selecting the\n desired settings. User can do OR operation of these macros for configuring\n multiple settings.\n\n@verbatim\n Macros                 |   Functionality\n -----------------------|----------------------------------------------\n BME280_SEL_OSR_PRESS   |   To set pressure oversampling.\n BME280_SEL_OSR_TEMP    |   To set temperature oversampling.\n BME280_SEL_OSR_HUM     |   To set humidity oversampling.\n BME280_SEL_FILTER      |   To set filter setting.\n BME280_SEL_STANDBY     |   To set standby duration setting.\n@endverbatim\n\n @return Result of API execution status\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_set_sensor_settings(
        desired_settings: u8,
        settings: *const bme280_settings,
        dev: *mut bme280_dev,
    ) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorSettings\n \\page bme280_api_bme280_get_sensor_settings bme280_get_sensor_settings\n \\code\n int8_t bme280_get_sensor_settings(struct bme280_settings *settings, struct bme280_dev *dev);\n \\endcode\n @details This API gets the oversampling, filter and standby duration\n (normal mode) settings from the sensor.\n\n @param[in] settings  : Structure instance of bme280_settings.\n @param[in,out] dev   : Structure instance of bme280_dev.\n\n @return Result of API execution status\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_get_sensor_settings(settings: *mut bme280_settings, dev: *mut bme280_dev) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorMode\n \\page bme280_api_bme280_set_sensor_mode bme280_set_sensor_mode\n \\code\n int8_t bme280_set_sensor_mode(uint8_t sensor_mode, const struct bme280_dev *dev);\n \\endcode\n @details This API sets the power mode of the sensor.\n\n @param[in] sensor_mode : Variable which contains the power mode to be set.\n @param[in] dev         : Structure instance of bme280_dev.\n\n@verbatim\n    sensor_mode       |      Macros\n ---------------------|-------------------------\n     0                | BME280_POWERMODE_SLEEP\n     1                | BME280_POWERMODE_FORCED\n     3                | BME280_POWERMODE_NORMAL\n@endverbatim\n\n @return Result of API execution status\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_set_sensor_mode(sensor_mode: u8, dev: *mut bme280_dev) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorMode\n \\page bme280_api_bme280_get_sensor_mode bme280_get_sensor_mode\n \\code\n int8_t bme280_get_sensor_mode(uint8_t *sensor_mode, struct bme280_dev *dev);\n \\endcode\n @details This API gets the power mode of the sensor.\n\n @param[out] sensor_mode : Pointer variable to store the power mode.\n @param[in] dev          : Structure instance of bme280_dev.\n\n@verbatim\n    sensor_mode       |      Macros\n ---------------------|-------------------------\n     0                | BME280_POWERMODE_SLEEP\n     1                | BME280_POWERMODE_FORCED\n     3                | BME280_POWERMODE_NORMAL\n@endverbatim\n\n @return Result of API execution status\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_get_sensor_mode(sensor_mode: *mut u8, dev: *mut bme280_dev) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSystem\n \\page bme280_api_bme280_soft_reset bme280_soft_reset\n \\code\n int8_t bme280_soft_reset(struct bme280_dev *dev);\n \\endcode\n @details This API soft-resets the sensor.\n\n @param[in,out] dev : Structure instance of bme280_dev.\n\n @return Result of API execution status.\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_soft_reset(dev: *mut bme280_dev) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorData\n \\page bme280_api_bme280_get_sensor_data bme280_get_sensor_data\n \\code\n int8_t bme280_get_sensor_data(uint8_t sensor_comp, struct bme280_data *comp_data, struct bme280_dev *dev);\n \\endcode\n @details This API reads the pressure, temperature and humidity data from the\n sensor, compensates the data and store it in the bme280_data structure\n instance passed by the user.\n\n @param[in] sensor_comp : Variable which selects which data to be read from\n the sensor.\n\n@verbatim\n sensor_comp |   Macros\n ------------|-------------------\n     1       | BME280_PRESS\n     2       | BME280_TEMP\n     4       | BME280_HUM\n     7       | BME280_ALL\n@endverbatim\n\n @param[out] comp_data : Structure instance of bme280_data.\n @param[in] dev        : Structure instance of bme280_dev.\n\n @return Result of API execution status\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_get_sensor_data(
        sensor_comp: u8,
        comp_data: *mut bme280_data,
        dev: *mut bme280_dev,
    ) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorData\n \\page bme280_api_bme280_compensate_data bme280_compensate_data\n \\code\n int8_t bme280_compensate_data(uint8_t sensor_comp,\n                             const struct bme280_uncomp_data *uncomp_data,\n                             struct bme280_data *comp_data,\n                             struct bme280_calib_data *calib_data);\n \\endcode\n @details This API is used to compensate the pressure and/or\n temperature and/or humidity data according to the component selected by the\n user.\n\n @param[in] sensor_comp : Used to select pressure and/or temperature and/or\n                          humidity.\n @param[in] uncomp_data : Contains the uncompensated pressure, temperature and\n                          humidity data.\n @param[out] comp_data  : Contains the compensated pressure and/or temperature\n                          and/or humidity data.\n @param[in] calib_data  : Pointer to bme280_calib_data\n\n @return Result of API execution status.\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_compensate_data(
        sensor_comp: u8,
        uncomp_data: *const bme280_uncomp_data,
        comp_data: *mut bme280_data,
        calib_data: *mut bme280_calib_data,
    ) -> i8;
}
unsafe extern "C" {
    #[doc = " \\ingroup bme280ApiSensorDelay\n \\page bme280_api_bme280_cal_meas_delay bme280_cal_meas_delay\n \\code\n uint32_t bme280_cal_meas_delay(uint32_t *max_delay, const struct bme280_settings *settings);\n \\endcode\n\n @details This API is used to calculate the maximum delay in microseconds required for the\n temperature/pressure/humidity(whichever are enabled) measurement to complete.\n The delay depends upon the number of sensors enabled and their oversampling configuration.\n\n @param[out] max_delay  : Delay required in microseconds.\n @param[in] settings    : Contains the oversampling configurations.\n\n @return Result of API execution status.\n\n @retval   0 -> Success.\n @retval > 0 -> Warning.\n @retval < 0 -> Fail.\n"]
    pub fn bme280_cal_meas_delay(max_delay: *mut u32, settings: *const bme280_settings) -> i8;
}
