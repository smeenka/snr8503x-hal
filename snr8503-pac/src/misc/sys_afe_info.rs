#[doc = "Register `SYS_AFE_INFO` reader"]
pub type R = crate::R<SysAfeInfoSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_afe_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysAfeInfoSpec;
impl crate::RegisterSpec for SysAfeInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_afe_info::R`](R) reader structure"]
impl crate::Readable for SysAfeInfoSpec {}
#[doc = "`reset()` method sets SYS_AFE_INFO to value 0"]
impl crate::Resettable for SysAfeInfoSpec {}
