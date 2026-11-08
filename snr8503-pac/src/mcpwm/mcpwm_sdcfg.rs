#[doc = "Register `MCPWM_SDCFG` reader"]
pub type R = crate::R<McpwmSdcfgSpec>;
#[doc = "Register `MCPWM_SDCFG` writer"]
pub type W = crate::W<McpwmSdcfgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_sdcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_sdcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmSdcfgSpec;
impl crate::RegisterSpec for McpwmSdcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_sdcfg::R`](R) reader structure"]
impl crate::Readable for McpwmSdcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_sdcfg::W`](W) writer structure"]
impl crate::Writable for McpwmSdcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_SDCFG to value 0"]
impl crate::Resettable for McpwmSdcfgSpec {}
