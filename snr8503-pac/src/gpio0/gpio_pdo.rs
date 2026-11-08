#[doc = "Register `GPIO_PDO` reader"]
pub type R = crate::R<GpioPdoSpec>;
#[doc = "Register `GPIO_PDO` writer"]
pub type W = crate::W<GpioPdoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_pdo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_pdo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioPdoSpec;
impl crate::RegisterSpec for GpioPdoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_pdo::R`](R) reader structure"]
impl crate::Readable for GpioPdoSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_pdo::W`](W) writer structure"]
impl crate::Writable for GpioPdoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_PDO to value 0"]
impl crate::Resettable for GpioPdoSpec {}
