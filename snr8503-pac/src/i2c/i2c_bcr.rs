#[doc = "Register `I2C_BCR` reader"]
pub type R = crate::R<I2cBcrSpec>;
#[doc = "Register `I2C_BCR` writer"]
pub type W = crate::W<I2cBcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cBcrSpec;
impl crate::RegisterSpec for I2cBcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_bcr::R`](R) reader structure"]
impl crate::Readable for I2cBcrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_bcr::W`](W) writer structure"]
impl crate::Writable for I2cBcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_BCR to value 0"]
impl crate::Resettable for I2cBcrSpec {}
