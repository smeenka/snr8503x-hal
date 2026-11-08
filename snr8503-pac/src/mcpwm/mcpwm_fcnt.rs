#[doc = "Register `MCPWM_FCNT` reader"]
pub type R = crate::R<McpwmFcntSpec>;
#[doc = "Register `MCPWM_FCNT` writer"]
pub type W = crate::W<McpwmFcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_fcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_fcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmFcntSpec;
impl crate::RegisterSpec for McpwmFcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_fcnt::R`](R) reader structure"]
impl crate::Readable for McpwmFcntSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_fcnt::W`](W) writer structure"]
impl crate::Writable for McpwmFcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_FCNT to value 0"]
impl crate::Resettable for McpwmFcntSpec {}
