#[doc = "Register `reg_key_slot_10_w0` reader"]
pub struct R(crate::R<REG_KEY_SLOT_10_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_KEY_SLOT_10_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_KEY_SLOT_10_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_KEY_SLOT_10_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "reg_key_slot_10_w0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_10_w0](index.html) module"]
pub struct REG_KEY_SLOT_10_W0_SPEC;
impl crate::RegisterSpec for REG_KEY_SLOT_10_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_key_slot_10_w0::R](R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_10_W0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets reg_key_slot_10_w0 to value 0"]
impl crate::Resettable for REG_KEY_SLOT_10_W0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}