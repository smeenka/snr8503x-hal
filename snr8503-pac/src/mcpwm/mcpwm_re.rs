#[doc = "Register `MCPWM_RE` reader"]
pub type R = crate::R<McpwmReSpec>;
#[doc = "Register `MCPWM_RE` writer"]
pub type W = crate::W<McpwmReSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_re::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_re::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmReSpec;
impl crate::RegisterSpec for McpwmReSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_re::R`](R) reader structure"]
impl crate::Readable for McpwmReSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_re::W`](W) writer structure"]
impl crate::Writable for McpwmReSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_RE to value 0"]
impl crate::Resettable for McpwmReSpec {}
