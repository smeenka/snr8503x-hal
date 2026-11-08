#[doc = "Register `MCPWM_EIF` reader"]
pub type R = crate::R<McpwmEifSpec>;
#[doc = "Register `MCPWM_EIF` writer"]
pub type W = crate::W<McpwmEifSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_eif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_eif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmEifSpec;
impl crate::RegisterSpec for McpwmEifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_eif::R`](R) reader structure"]
impl crate::Readable for McpwmEifSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_eif::W`](W) writer structure"]
impl crate::Writable for McpwmEifSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_EIF to value 0"]
impl crate::Resettable for McpwmEifSpec {}
