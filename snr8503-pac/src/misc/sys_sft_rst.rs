#[doc = "Register `SYS_SFT_RST` reader"]
pub type R = crate::R<SysSftRstSpec>;
#[doc = "Register `SYS_SFT_RST` writer"]
pub type W = crate::W<SysSftRstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sft_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sft_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSftRstSpec;
impl crate::RegisterSpec for SysSftRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sft_rst::R`](R) reader structure"]
impl crate::Readable for SysSftRstSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_sft_rst::W`](W) writer structure"]
impl crate::Writable for SysSftRstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_SFT_RST to value 0"]
impl crate::Resettable for SysSftRstSpec {}
