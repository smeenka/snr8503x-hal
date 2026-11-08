#[doc = "Register `IWDG_WTH` reader"]
pub type R = crate::R<IwdgWthSpec>;
#[doc = "Register `IWDG_WTH` writer"]
pub type W = crate::W<IwdgWthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_wth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_wth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgWthSpec;
impl crate::RegisterSpec for IwdgWthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_wth::R`](R) reader structure"]
impl crate::Readable for IwdgWthSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_wth::W`](W) writer structure"]
impl crate::Writable for IwdgWthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_WTH to value 0"]
impl crate::Resettable for IwdgWthSpec {}
