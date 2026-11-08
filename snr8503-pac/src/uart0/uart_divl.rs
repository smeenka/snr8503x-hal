#[doc = "Register `UART_DIVL` reader"]
pub type R = crate::R<UartDivlSpec>;
#[doc = "Register `UART_DIVL` writer"]
pub type W = crate::W<UartDivlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_divl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_divl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartDivlSpec;
impl crate::RegisterSpec for UartDivlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_divl::R`](R) reader structure"]
impl crate::Readable for UartDivlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_divl::W`](W) writer structure"]
impl crate::Writable for UartDivlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_DIVL to value 0"]
impl crate::Resettable for UartDivlSpec {}
