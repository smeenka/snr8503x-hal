#[doc = "Register `MCPWM_CNT1` reader"]
pub type R = crate::R<McpwmCnt1Spec>;
#[doc = "Register `MCPWM_CNT1` writer"]
pub type W = crate::W<McpwmCnt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McpwmCnt1Spec;
impl crate::RegisterSpec for McpwmCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm_cnt1::R`](R) reader structure"]
impl crate::Readable for McpwmCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcpwm_cnt1::W`](W) writer structure"]
impl crate::Writable for McpwmCnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM_CNT1 to value 0"]
impl crate::Resettable for McpwmCnt1Spec {}
