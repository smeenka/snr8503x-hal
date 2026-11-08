#[doc = "Register `GPIO_PDI` reader"]
pub type R = crate::R<GpioPdiSpec>;
#[doc = "Register `GPIO_PDI` writer"]
pub type W = crate::W<GpioPdiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pdi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pdi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPdiSpec;
impl crate::RegisterSpec for GpioPdiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pdi::R`](R) reader structure"]
impl crate::Readable for GpioPdiSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pdi::W`](W) writer structure"]
impl crate::Writable for GpioPdiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_PDI to value 0"]
impl crate::Resettable for GpioPdiSpec {}
