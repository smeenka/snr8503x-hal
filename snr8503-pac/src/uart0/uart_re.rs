#[doc = "Register `UART_RE` reader"]
pub type R = crate::R<UartReSpec>;
#[doc = "Register `UART_RE` writer"]
pub type W = crate::W<UartReSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_re::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_re::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartReSpec;
impl crate::RegisterSpec for UartReSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_re::R`](R) reader structure"]
impl crate::Readable for UartReSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_re::W`](W) writer structure"]
impl crate::Writable for UartReSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_RE to value 0"]
impl crate::Resettable for UartReSpec {}
