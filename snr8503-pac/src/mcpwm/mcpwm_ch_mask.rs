#[doc = "Register `MCPWM_CH_MASK` reader"]
pub type R = crate::R<McpwmChMaskSpec>;
#[doc = "Register `MCPWM_CH_MASK` writer"]
pub type W = crate::W<McpwmChMaskSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_ch_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_ch_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmChMaskSpec;
impl crate::RegisterSpec for McpwmChMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_ch_mask::R`](R) reader structure"]
impl crate::Readable for McpwmChMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_ch_mask::W`](W) writer structure"]
impl crate::Writable for McpwmChMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_CH_MASK to value 0"]
impl crate::Resettable for McpwmChMaskSpec {}
