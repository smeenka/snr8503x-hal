use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;

pub struct Delay {
    syst: SYST,
    fac_us: u32,
    fac_ms: u32,
}

impl Delay {
    /// Initialize delay provider
    /// 
    /// `sysclk` is the system clock frequency in Hz
    pub fn new(mut syst: SYST, sysclk: u32) -> Self {
        // C: SysTick->CTRL&=~(1<<2); // SYSTICK使用外部时钟源，1/8 HCLK
        syst.set_clock_source(SystClkSource::External);
        
        // C: fac_us=SYSCLK/8;
        let fac_us = sysclk / 8 / 1_000_000;
        
        // C: fac_ms=(u16)fac_us*1000;
        let fac_ms = fac_us * 1000;

        Delay {
            syst,
            fac_us,
            fac_ms,
        }
    }

    /// Delay n microseconds
    pub fn delay_us(&mut self, nus: u32) {
        // C: SysTick->LOAD=nus*fac_us;
        let ticks = nus * self.fac_us;
        self.syst.set_reload(ticks);
        
        // C: SysTick->VAL=0x00;
        self.syst.clear_current();
        
        // C: SysTick->CTRL=0x01;
        self.syst.enable_counter();
        
        // C: while((temp&0x01)&&!(temp&(1<<16)));
        while !self.syst.has_wrapped() {}
        
        // C: SysTick->CTRL=0x00;
        self.syst.disable_counter();
        
        // C: SysTick->VAL =0X00;
        self.syst.clear_current();
    }

    /// Delay n milliseconds (internal helper)
    fn delay_xms(&mut self, nms: u32) {
        // C: SysTick->LOAD=(u32)nms*fac_ms;
        let ticks = nms * self.fac_ms;
        self.syst.set_reload(ticks);
        
        // C: SysTick->VAL =0x00;
        self.syst.clear_current();
        
        // C: SysTick->CTRL=0x01;
        self.syst.enable_counter();
        
        // C: while((temp&0x01)&&!(temp&(1<<16)));
        while !self.syst.has_wrapped() {}
        
        // C: SysTick->CTRL=0x00;
        self.syst.disable_counter();
        
        // C: SysTick->VAL =0X00;
        self.syst.clear_current();
    }

    /// Delay n milliseconds
    /// 
    /// Handles long delays by breaking them down
    pub fn delay_ms(&mut self, nms: u32) {
        // C: u8 repeat=nms/540;
        let repeat = nms / 540;
        let remain = nms % 540;
        
        for _ in 0..repeat {
            self.delay_xms(540);
        }
        
        if remain > 0 {
            self.delay_xms(remain);
        }
    }
}
