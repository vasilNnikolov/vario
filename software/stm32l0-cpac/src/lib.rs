pub mod c_bindings;

macro_rules! peripheral {
    ($name:ty, $addr:expr) => {
        impl $name {
            pub fn new() -> &'static mut $name {
                unsafe { &mut *($addr as *mut $name) }
            }
        }
    };
}

peripheral!(c_bindings::RCC_TypeDef, c_bindings::RCC_BASE);

fn test_x() {
    let a = c_bindings::RCC_TypeDef::new();
    a.CR |= c_bindings::RCC_CR_PLLRDY_Msk;
    while (a.CIFR & c_bindings::RCC_CIFR_CSSLSEF_Msk) >> c_bindings::RCC_CIFR_CSSLSEF_Pos == 0 {}
}
