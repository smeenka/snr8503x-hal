#[doc = "Register `SYS_AFE_REG5` reader"]
pub type R = crate::R<SysAfeReg5Spec>;
#[doc = "Register `SYS_AFE_REG5` writer"]
pub type W = crate::W<SysAfeReg5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_afe_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysAfeReg5Spec;
impl crate::RegisterSpec for SysAfeReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_afe_reg5::R`](R) reader structure"]
impl crate::Readable for SysAfeReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_afe_reg5::W`](W) writer structure"]
impl crate::Writable for SysAfeReg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_AFE_REG5 to value 0"]
impl crate::Resettable for SysAfeReg5Spec {}
