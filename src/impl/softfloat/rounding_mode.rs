use fpa_specr_berkeley_softfloat::*;

use crate::RoundingMode;

impl RoundingMode {
    #[inline]
    pub(crate) fn as_berkeley_softfloat(&self) -> u8 {
        match self {
            Self::NearestTiesEven => softfloat_round_near_even,
            Self::TowardPosInf => softfloat_round_max,
            Self::TowardNegInf => softfloat_round_min,
            Self::TowardZero => softfloat_round_minMag,
        }
    }
}
