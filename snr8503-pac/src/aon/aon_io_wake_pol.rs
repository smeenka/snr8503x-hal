#[doc = "Register `AON_IO_WAKE_POL` reader"]
pub type R = crate::R<AonIoWakePolSpec>;
#[doc = "Register `AON_IO_WAKE_POL` writer"]
pub type W = crate::W<AonIoWakePolSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_io_wake_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_io_wake_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonIoWakePolSpec;
impl crate::RegisterSpec for AonIoWakePolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_io_wake_pol::R`](R) reader structure"]
impl crate::Readable for AonIoWakePolSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_io_wake_pol::W`](W) writer structure"]
impl crate::Writable for AonIoWakePolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_IO_WAKE_POL to value 0"]
impl crate::Resettable for AonIoWakePolSpec {}
