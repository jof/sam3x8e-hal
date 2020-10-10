extern crate embedded_hal as hal;
use sam3x8e::Peripherals;
use crate::pmc::Clocks;
use crate::gpio::PWMPins;
use crate::pwm::Channel;
use crate::pwm::PWM;
use crate::hal::Pwm;

const PIO_WPKEY: u32 = 0x50494F; // "PIO"
pub struct PWMPin<'a> {
    peripherals: &'a Peripherals,
    clocks: &'a Clocks,
    pin: PWMPins,
    channel: Channel,
}

impl<'a> PWMPin<'a> {
    pub fn new(peripherals: &'a Peripherals, clocks: &'a Clocks, pin: PWMPins) -> Self {
        let channel = match pin {
            PWMPins::PA0 => Channel::CHID3,
            PWMPins::PA8  => Channel::CHID0,
            PWMPins::PA9  => Channel::CHID3,
            PWMPins::PA12 => Channel::CHID1,
            PWMPins::PA13 => Channel::CHID2,
            PWMPins::PA19 => Channel::CHID1,
            PWMPins::PA20 => Channel::CHID2,
            PWMPins::PA21 => Channel::CHID0,
            PWMPins::PB6 => Channel::CHID4,
            PWMPins::PB7 => Channel::CHID5,
            PWMPins::PB8 => Channel::CHID6,
            PWMPins::PB9 => Channel::CHID7,
            PWMPins::PB12 => Channel::CHID0,
            PWMPins::PB13 => Channel::CHID1,
            PWMPins::PB14 => Channel::CHID2,
            PWMPins::PB15 => Channel::CHID3,
            PWMPins::PB16 => Channel::CHID0,
            PWMPins::PB17 => Channel::CHID1,
            PWMPins::PB18 => Channel::CHID2,
            PWMPins::PB19 => Channel::CHID3,
            PWMPins::PC2 => Channel::CHID0,
            PWMPins::PC3 => Channel::CHID0,
            PWMPins::PC4 => Channel::CHID1,
            PWMPins::PC5 => Channel::CHID1,
            PWMPins::PC6 => Channel::CHID2,
            PWMPins::PC7 => Channel::CHID2,
            PWMPins::PC8 => Channel::CHID3,
            PWMPins::PC9 => Channel::CHID3,
            PWMPins::PC18 => Channel::CHID6,
            PWMPins::PC19 => Channel::CHID5,
            PWMPins::PC20 => Channel::CHID4,
            PWMPins::PC21 => Channel::CHID4,
            PWMPins::PC22 => Channel::CHID5,
            PWMPins::PC23 => Channel::CHID6,
            PWMPins::PC24 => Channel::CHID7,
        };
        PWMPin {
            peripherals: peripherals,
            clocks: clocks,
            pin: pin,
            channel: channel,
        }
    }
}
impl<'a> hal::PwmPin for PWMPin<'a> {
    type Duty = f32; // 0.0 .. 1.0
    fn enable(&mut self) {
        self.peripherals.PIOA.wpmr.write_with_zero(|w| unsafe { w.wpkey().bits(PIO_WPKEY).wpen().clear_bit() });
        let setup_pioa_for_pwm = |bits| {
            self.peripherals.PIOA.absr.modify(|r, w| unsafe { w.bits(r.bits() | bits)});
            self.peripherals.PIOA.puer.write_with_zero(|w| unsafe {w.bits(bits)});
            self.peripherals.PIOA.oer.write_with_zero(|w| unsafe {w.bits(bits)});
        };
        let setup_piob_for_pwm = |bits| {
            self.peripherals.PIOB.absr.modify(|r, w| unsafe { w.bits(r.bits() | bits)});
            self.peripherals.PIOB.puer.write_with_zero(|w| unsafe {w.bits(bits)});
            self.peripherals.PIOB.oer.write_with_zero(|w| unsafe {w.bits(bits)});
        };
        let setup_pioc_for_pwm = |bits| {
            self.peripherals.PIOC.absr.modify(|r, w| unsafe { w.bits(r.bits() | bits)});
            self.peripherals.PIOC.puer.write_with_zero(|w| unsafe {w.bits(bits)});
            self.peripherals.PIOC.oer.write_with_zero(|w| unsafe {w.bits(bits)});
        };
        // let setup_piod_for_pwm = |bits| {
        //     self.peripherals.PIOD.absr.modify(|r, w| unsafe { w.bits(r.bits() | bits)});
        //     self.peripherals.PIOD.puer.write_with_zero(|w| unsafe {w.bits(bits)});
        //     self.peripherals.PIOD.oer.write_with_zero(|w| unsafe {w.bits(bits)});
        // };
        match self.pin {
            PWMPins::PA0 => setup_pioa_for_pwm(0b1),
            PWMPins::PA8  => setup_pioa_for_pwm(0b1 << 8),
            PWMPins::PA9  => setup_pioa_for_pwm(0b1 << 9),
            PWMPins::PA12 => setup_pioa_for_pwm(0b1 << 12),
            PWMPins::PA13 => setup_pioa_for_pwm(0b1 << 13),
            PWMPins::PA19 => setup_pioa_for_pwm(0b1 << 19),
            PWMPins::PA20 => setup_pioa_for_pwm(0b1 << 20),
            PWMPins::PA21 => setup_pioa_for_pwm(0b1 << 21),
            PWMPins::PB6 => setup_piob_for_pwm(0b1 << 6),
            PWMPins::PB7 => setup_piob_for_pwm(0b1 << 7),
            PWMPins::PB8 => setup_piob_for_pwm(0b1 << 8),
            PWMPins::PB9 => setup_piob_for_pwm(0b1 << 9),
            PWMPins::PB12 => setup_piob_for_pwm(0b1 << 12),
            PWMPins::PB13 => setup_piob_for_pwm(0b1 << 13),
            PWMPins::PB14 => setup_piob_for_pwm(0b1 << 14),
            PWMPins::PB15 => setup_piob_for_pwm(0b1 << 15),
            PWMPins::PB16 => setup_piob_for_pwm(0b1 << 16),
            PWMPins::PB17 => setup_piob_for_pwm(0b1 << 17),
            PWMPins::PB18 => setup_piob_for_pwm(0b1 << 18),
            PWMPins::PB19 => setup_piob_for_pwm(0b1 << 19),
            PWMPins::PC2 => setup_pioc_for_pwm(0b1 << 2),
            PWMPins::PC3 => setup_pioc_for_pwm(0b1 << 3),
            PWMPins::PC4 => setup_pioc_for_pwm(0b1 << 4),
            PWMPins::PC5 => setup_pioc_for_pwm(0b1 << 5),
            PWMPins::PC6 => setup_pioc_for_pwm(0b1 << 6),
            PWMPins::PC7 => setup_pioc_for_pwm(0b1 << 7),
            PWMPins::PC8 => setup_pioc_for_pwm(0b1 << 8),
            PWMPins::PC9 => setup_pioc_for_pwm(0b1 << 9),
            PWMPins::PC18 => setup_pioc_for_pwm(0b1 << 18),
            PWMPins::PC19 => setup_pioc_for_pwm(0b1 << 19),
            PWMPins::PC20 => setup_pioc_for_pwm(0b1 << 20),
            PWMPins::PC21 => setup_pioc_for_pwm(0b1 << 21),
            PWMPins::PC22 => setup_pioc_for_pwm(0b1 << 22),
            PWMPins::PC23 => setup_pioc_for_pwm(0b1 << 23),
            PWMPins::PC24 => setup_pioc_for_pwm(0b1 << 24),
            _ => unreachable!()
        }
    }

    fn disable(&mut self) {
        self.peripherals.PIOA.wpmr.write_with_zero(|w| unsafe { w.wpkey().bits(PIO_WPKEY).wpen().clear_bit() });
        let teardown_pioa_for_pwm = |bits| {
            self.peripherals.PIOA.odr.write_with_zero(|w| unsafe {w.bits(bits)});
        };
        let teardown_piob_for_pwm = |bits| {
            self.peripherals.PIOB.odr.write_with_zero(|w| unsafe {w.bits(bits)});
        };
        let teardown_pioc_for_pwm = |bits| {
            self.peripherals.PIOC.odr.write_with_zero(|w| unsafe {w.bits(bits)});
        };
        // let teardown_piod_for_pwm = |bits| {
        //     self.peripherals.PIOD.odr.write_with_zero(|w| unsafe {w.bits(bits)});
        // };
        match self.pin {
            PWMPins::PA0 => teardown_pioa_for_pwm(0b1),
            PWMPins::PA8  => teardown_pioa_for_pwm(0b1 << 8),
            PWMPins::PA9  => teardown_pioa_for_pwm(0b1 << 9),
            PWMPins::PA12 => teardown_pioa_for_pwm(0b1 << 12),
            PWMPins::PA13 => teardown_pioa_for_pwm(0b1 << 13),
            PWMPins::PA19 => teardown_pioa_for_pwm(0b1 << 19),
            PWMPins::PA20 => teardown_pioa_for_pwm(0b1 << 20),
            PWMPins::PA21 => teardown_pioa_for_pwm(0b1 << 21),
            PWMPins::PB6 => teardown_piob_for_pwm(0b1 << 6),
            PWMPins::PB7 => teardown_piob_for_pwm(0b1 << 7),
            PWMPins::PB8 => teardown_piob_for_pwm(0b1 << 8),
            PWMPins::PB9 => teardown_piob_for_pwm(0b1 << 9),
            PWMPins::PB12 => teardown_piob_for_pwm(0b1 << 12),
            PWMPins::PB13 => teardown_piob_for_pwm(0b1 << 13),
            PWMPins::PB14 => teardown_piob_for_pwm(0b1 << 14),
            PWMPins::PB15 => teardown_piob_for_pwm(0b1 << 15),
            PWMPins::PB16 => teardown_piob_for_pwm(0b1 << 16),
            PWMPins::PB17 => teardown_piob_for_pwm(0b1 << 17),
            PWMPins::PB18 => teardown_piob_for_pwm(0b1 << 18),
            PWMPins::PB19 => teardown_piob_for_pwm(0b1 << 19),
            PWMPins::PC2 => teardown_pioc_for_pwm(0b1 << 2),
            PWMPins::PC3 => teardown_pioc_for_pwm(0b1 << 3),
            PWMPins::PC4 => teardown_pioc_for_pwm(0b1 << 4),
            PWMPins::PC5 => teardown_pioc_for_pwm(0b1 << 5),
            PWMPins::PC6 => teardown_pioc_for_pwm(0b1 << 6),
            PWMPins::PC7 => teardown_pioc_for_pwm(0b1 << 7),
            PWMPins::PC8 => teardown_pioc_for_pwm(0b1 << 8),
            PWMPins::PC9 => teardown_pioc_for_pwm(0b1 << 9),
            PWMPins::PC18 => teardown_pioc_for_pwm(0b1 << 18),
            PWMPins::PC19 => teardown_pioc_for_pwm(0b1 << 19),
            PWMPins::PC20 => teardown_pioc_for_pwm(0b1 << 20),
            PWMPins::PC21 => teardown_pioc_for_pwm(0b1 << 21),
            PWMPins::PC22 => teardown_pioc_for_pwm(0b1 << 22),
            PWMPins::PC23 => teardown_pioc_for_pwm(0b1 << 23),
            PWMPins::PC24 => teardown_pioc_for_pwm(0b1 << 24),
            _ => unreachable!()
        }
    }

    fn get_duty(&self) -> Self::Duty {
        return PWM::new(self.peripherals, self.clocks).get_duty(self.channel);
    }

    fn get_max_duty(&self) -> Self::Duty {
        return 1.0
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let cprd = match self.channel {
            Channel::CHID0 => self.peripherals.PWM.cprd0.read().cprd().bits(),
            Channel::CHID1 => self.peripherals.PWM.cprd1.read().cprd().bits(),
            Channel::CHID2 => self.peripherals.PWM.cprd2.read().cprd().bits(),
            Channel::CHID3 => self.peripherals.PWM.cprd3.read().cprd().bits(),
            Channel::CHID4 => self.peripherals.PWM.cprd4.read().cprd().bits(),
            Channel::CHID5 => self.peripherals.PWM.cprd5.read().cprd().bits(),
            Channel::CHID6 => self.peripherals.PWM.cprd6.read().cprd().bits(),
            Channel::CHID7 => self.peripherals.PWM.cprd7.read().cprd().bits(),
        } as f32;
        let cdty = (duty * cprd) as u32;
        match self.channel {
            Channel::CHID0 => self.peripherals.PWM.cdty0.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID1 => self.peripherals.PWM.cdty1.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID2 => self.peripherals.PWM.cdty2.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID3 => self.peripherals.PWM.cdty3.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID4 => self.peripherals.PWM.cdty4.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID5 => self.peripherals.PWM.cdty5.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID6 => self.peripherals.PWM.cdty6.write(|w| unsafe { w.cdty().bits(cdty) }),
            Channel::CHID7 => self.peripherals.PWM.cdty7.write(|w| unsafe { w.cdty().bits(cdty) }),
        };
    }
}
