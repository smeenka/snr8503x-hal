#[doc = "Register `IWDG_RTH` reader"]
pub type R = crate::R<IwdgRthSpec>;
#[doc = "Register `IWDG_RTH` writer"]
pub type W = crate::W<IwdgRthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_rth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_rth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgRthSpec;
impl crate::RegisterSpec for IwdgRthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_rth::R`](R) reader structure"]
impl crate::Readable for IwdgRthSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_rth::W`](W) writer structure"]
impl crate::Writable for IwdgRthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_RTH to value 0"]
impl crate::Resettable for IwdgRthSpec {}
