#[doc = "Register `UART_DIVH` reader"]
pub type R = crate::R<UartDivhSpec>;
#[doc = "Register `UART_DIVH` writer"]
pub type W = crate::W<UartDivhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_divh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_divh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDivhSpec;
impl crate::RegisterSpec for UartDivhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_divh::R`](R) reader structure"]
impl crate::Readable for UartDivhSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_divh::W`](W) writer structure"]
impl crate::Writable for UartDivhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_DIVH to value 0"]
impl crate::Resettable for UartDivhSpec {}
