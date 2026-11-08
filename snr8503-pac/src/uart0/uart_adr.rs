#[doc = "Register `UART_ADR` reader"]
pub type R = crate::R<UartAdrSpec>;
#[doc = "Register `UART_ADR` writer"]
pub type W = crate::W<UartAdrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartAdrSpec;
impl crate::RegisterSpec for UartAdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_adr::R`](R) reader structure"]
impl crate::Readable for UartAdrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_adr::W`](W) writer structure"]
impl crate::Writable for UartAdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_ADR to value 0"]
impl crate::Resettable for UartAdrSpec {}
