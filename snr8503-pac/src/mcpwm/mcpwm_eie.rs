#[doc = "Register `MCPWM_EIE` reader"]
pub type R = crate::R<McpwmEieSpec>;
#[doc = "Register `MCPWM_EIE` writer"]
pub type W = crate::W<McpwmEieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_eie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_eie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmEieSpec;
impl crate::RegisterSpec for McpwmEieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_eie::R`](R) reader structure"]
impl crate::Readable for McpwmEieSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_eie::W`](W) writer structure"]
impl crate::Writable for McpwmEieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_EIE to value 0"]
impl crate::Resettable for McpwmEieSpec {}
