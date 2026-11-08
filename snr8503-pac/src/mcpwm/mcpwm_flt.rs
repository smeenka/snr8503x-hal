#[doc = "Register `MCPWM_FLT` reader"]
pub type R = crate::R<McpwmFltSpec>;
#[doc = "Register `MCPWM_FLT` writer"]
pub type W = crate::W<McpwmFltSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_flt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_flt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmFltSpec;
impl crate::RegisterSpec for McpwmFltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_flt::R`](R) reader structure"]
impl crate::Readable for McpwmFltSpec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_flt::W`](W) writer structure"]
impl crate::Writable for McpwmFltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_FLT to value 0"]
impl crate::Resettable for McpwmFltSpec {}
