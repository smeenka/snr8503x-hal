#[doc = "Register `UART_STT` reader"]
pub type R = crate::R<UartSttSpec>;
#[doc = "Register `UART_STT` writer"]
pub type W = crate::W<UartSttSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_stt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_stt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSttSpec;
impl crate::RegisterSpec for UartSttSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_stt::R`](R) reader structure"]
impl crate::Readable for UartSttSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_stt::W`](W) writer structure"]
impl crate::Writable for UartSttSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_STT to value 0"]
impl crate::Resettable for UartSttSpec {}
