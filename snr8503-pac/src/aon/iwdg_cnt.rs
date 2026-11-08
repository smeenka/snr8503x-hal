#[doc = "Register `IWDG_CNT` reader"]
pub type R = crate::R<IwdgCntSpec>;
#[doc = "Register `IWDG_CNT` writer"]
pub type W = crate::W<IwdgCntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgCntSpec;
impl crate::RegisterSpec for IwdgCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_cnt::R`](R) reader structure"]
impl crate::Readable for IwdgCntSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_cnt::W`](W) writer structure"]
impl crate::Writable for IwdgCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_CNT to value 0"]
impl crate::Resettable for IwdgCntSpec {}
