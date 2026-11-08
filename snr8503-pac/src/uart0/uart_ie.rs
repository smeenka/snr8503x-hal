#[doc = "Register `UART_IE` reader"]
pub type R = crate::R<UartIeSpec>;
#[doc = "Register `UART_IE` writer"]
pub type W = crate::W<UartIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartIeSpec;
impl crate::RegisterSpec for UartIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ie::R`](R) reader structure"]
impl crate::Readable for UartIeSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_ie::W`](W) writer structure"]
impl crate::Writable for UartIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_IE to value 0"]
impl crate::Resettable for UartIeSpec {}
