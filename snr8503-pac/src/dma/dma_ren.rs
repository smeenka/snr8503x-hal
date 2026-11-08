#[doc = "Register `DMA_REN[%s]` reader"]
pub type R = crate::R<DmaRenSpec>;
#[doc = "Register `DMA_REN[%s]` writer"]
pub type W = crate::W<DmaRenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRenSpec;
impl crate::RegisterSpec for DmaRenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ren::R`](R) reader structure"]
impl crate::Readable for DmaRenSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_ren::W`](W) writer structure"]
impl crate::Writable for DmaRenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_REN[%s] to value 0"]
impl crate::Resettable for DmaRenSpec {}
