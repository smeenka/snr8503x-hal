#[doc = "Register `UART_BUFF` reader"]
pub type R = crate::R<UartBuffSpec>;
#[doc = "Register `UART_BUFF` writer"]
pub type W = crate::W<UartBuffSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_buff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_buff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartBuffSpec;
impl crate::RegisterSpec for UartBuffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_buff::R`](R) reader structure"]
impl crate::Readable for UartBuffSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_buff::W`](W) writer structure"]
impl crate::Writable for UartBuffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_BUFF to value 0"]
impl crate::Resettable for UartBuffSpec {}
