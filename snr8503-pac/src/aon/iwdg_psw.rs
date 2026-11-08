#[doc = "Register `IWDG_PSW` reader"]
pub type R = crate::R<IwdgPswSpec>;
#[doc = "Register `IWDG_PSW` writer"]
pub type W = crate::W<IwdgPswSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_psw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_psw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgPswSpec;
impl crate::RegisterSpec for IwdgPswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_psw::R`](R) reader structure"]
impl crate::Readable for IwdgPswSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_psw::W`](W) writer structure"]
impl crate::Writable for IwdgPswSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_PSW to value 0"]
impl crate::Resettable for IwdgPswSpec {}
