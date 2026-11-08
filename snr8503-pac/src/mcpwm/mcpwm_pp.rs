#[doc = "Register `MCPWM_PP` reader"]
pub type R = crate::R<McpwmPpSpec>;
#[doc = "Register `MCPWM_PP` writer"]
pub type W = crate::W<McpwmPpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_pp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_pp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmPpSpec;
impl crate::RegisterSpec for McpwmPpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_pp::R`](R) reader structure"]
impl crate::Readable for McpwmPpSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_pp::W`](W) writer structure"]
impl crate::Writable for McpwmPpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_PP to value 0"]
impl crate::Resettable for McpwmPpSpec {}
