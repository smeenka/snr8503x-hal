#[doc = "Register `MCPWM_UPDATE` reader"]
pub type R = crate::R<McpwmUpdateSpec>;
#[doc = "Register `MCPWM_UPDATE` writer"]
pub type W = crate::W<McpwmUpdateSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmUpdateSpec;
impl crate::RegisterSpec for McpwmUpdateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_update::R`](R) reader structure"]
impl crate::Readable for McpwmUpdateSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_update::W`](W) writer structure"]
impl crate::Writable for McpwmUpdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_UPDATE to value 0"]
impl crate::Resettable for McpwmUpdateSpec {}
