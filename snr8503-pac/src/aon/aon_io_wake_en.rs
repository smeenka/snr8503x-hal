#[doc = "Register `AON_IO_WAKE_EN` reader"]
pub type R = crate::R<AonIoWakeEnSpec>;
#[doc = "Register `AON_IO_WAKE_EN` writer"]
pub type W = crate::W<AonIoWakeEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_io_wake_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_io_wake_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonIoWakeEnSpec;
impl crate::RegisterSpec for AonIoWakeEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_io_wake_en::R`](R) reader structure"]
impl crate::Readable for AonIoWakeEnSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_io_wake_en::W`](W) writer structure"]
impl crate::Writable for AonIoWakeEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_IO_WAKE_EN to value 0"]
impl crate::Resettable for AonIoWakeEnSpec {}
