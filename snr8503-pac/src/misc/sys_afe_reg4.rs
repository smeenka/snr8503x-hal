#[doc = "Register `SYS_AFE_REG4` reader"]
pub type R = crate::R<SysAfeReg4Spec>;
#[doc = "Register `SYS_AFE_REG4` writer"]
pub type W = crate::W<SysAfeReg4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysAfeReg4Spec;
impl crate::RegisterSpec for SysAfeReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_afe_reg4::R`](R) reader structure"]
impl crate::Readable for SysAfeReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_afe_reg4::W`](W) writer structure"]
impl crate::Writable for SysAfeReg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_AFE_REG4 to value 0"]
impl crate::Resettable for SysAfeReg4Spec {}
