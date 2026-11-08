#[doc = "Register `DMA_DADR[%s]` reader"]
pub type R = crate::R<DmaDadrSpec>;
#[doc = "Register `DMA_DADR[%s]` writer"]
pub type W = crate::W<DmaDadrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaDadrSpec;
impl crate::RegisterSpec for DmaDadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_dadr::R`](R) reader structure"]
impl crate::Readable for DmaDadrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_dadr::W`](W) writer structure"]
impl crate::Writable for DmaDadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_DADR[%s] to value 0"]
impl crate::Resettable for DmaDadrSpec {}
