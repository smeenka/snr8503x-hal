#[doc = "Register `FLASH_CFG` reader"]
pub type R = crate::R<FlashCfgSpec>;
#[doc = "Register `FLASH_CFG` writer"]
pub type W = crate::W<FlashCfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCfgSpec;
impl crate::RegisterSpec for FlashCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_cfg::R`](R) reader structure"]
impl crate::Readable for FlashCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_cfg::W`](W) writer structure"]
impl crate::Writable for FlashCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_CFG to value 0"]
impl crate::Resettable for FlashCfgSpec {}
