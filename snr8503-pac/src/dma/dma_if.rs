#[doc = "Register `DMA_IF` reader"]
pub type R = crate::R<DmaIfSpec>;
#[doc = "Register `DMA_IF` writer"]
pub type W = crate::W<DmaIfSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIfSpec;
impl crate::RegisterSpec for DmaIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_if::R`](R) reader structure"]
impl crate::Readable for DmaIfSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_if::W`](W) writer structure"]
impl crate::Writable for DmaIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_IF to value 0"]
impl crate::Resettable for DmaIfSpec {}
