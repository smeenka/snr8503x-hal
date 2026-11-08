#[doc = "Register `GPIO_PIE` reader"]
pub type R = crate::R<GpioPieSpec>;
#[doc = "Register `GPIO_PIE` writer"]
pub type W = crate::W<GpioPieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPieSpec;
impl crate::RegisterSpec for GpioPieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pie::R`](R) reader structure"]
impl crate::Readable for GpioPieSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pie::W`](W) writer structure"]
impl crate::Writable for GpioPieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_PIE to value 0"]
impl crate::Resettable for GpioPieSpec {}
