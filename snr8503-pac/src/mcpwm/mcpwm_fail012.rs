#[doc = "Register `MCPWM_FAIL012` reader"]
pub type R = crate::R<McpwmFail012Spec>;
#[doc = "Register `MCPWM_FAIL012` writer"]
pub type W = crate::W<McpwmFail012Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_fail012::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_fail012::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmFail012Spec;
impl crate::RegisterSpec for McpwmFail012Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_fail012::R`](R) reader structure"]
impl crate::Readable for McpwmFail012Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_fail012::W`](W) writer structure"]
impl crate::Writable for McpwmFail012Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_FAIL012 to value 0"]
impl crate::Resettable for McpwmFail012Spec {}
