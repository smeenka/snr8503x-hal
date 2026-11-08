#[doc = "Register `IWDG_CLR` reader"]
pub type R = crate::R<IwdgClrSpec>;
#[doc = "Register `IWDG_CLR` writer"]
pub type W = crate::W<IwdgClrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdg_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdgClrSpec;
impl crate::RegisterSpec for IwdgClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iwdg_clr::R`](R) reader structure"]
impl crate::Readable for IwdgClrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdg_clr::W`](W) writer structure"]
impl crate::Writable for IwdgClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IWDG_CLR to value 0"]
impl crate::Resettable for IwdgClrSpec {}
