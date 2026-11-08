#[doc = "Register `SYS_CLK_FEN` reader"]
pub type R = crate::R<SysClkFenSpec>;
#[doc = "Register `SYS_CLK_FEN` writer"]
pub type W = crate::W<SysClkFenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "peripheral clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_clk_fen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_clk_fen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysClkFenSpec;
impl crate::RegisterSpec for SysClkFenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_clk_fen::R`](R) reader structure"]
impl crate::Readable for SysClkFenSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_clk_fen::W`](W) writer structure"]
impl crate::Writable for SysClkFenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CLK_FEN to value 0"]
impl crate::Resettable for SysClkFenSpec {}
