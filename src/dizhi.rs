use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::wuxing::WuXing;

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum DiZhi {
    #[default]
    子,
    丑,
    寅,
    卯,
    辰,
    巳,
    午,
    未,
    申,
    酉,
    戌,
    亥,
}

impl DiZhi {
    fn num(&self) -> u8 {
        match self {
            DiZhi::子 => 1,
            DiZhi::丑 => 2,
            DiZhi::寅 => 3,
            DiZhi::卯 => 4,
            DiZhi::辰 => 5,
            DiZhi::巳 => 6,
            DiZhi::午 => 7,
            DiZhi::未 => 8,
            DiZhi::申 => 9,
            DiZhi::酉 => 10,
            DiZhi::戌 => 11,
            DiZhi::亥 => 12,
        }
    }

    /// 五行
    pub fn wu_xing(&self) -> WuXing {
        let mut n = (self.num() + 10) % 12;
        if n == 0 {
            n = 12;
        }

        if n % 3 == 0 {
            return WuXing::土;
        }

        let n = n / 3 + n / 7 + 1;
        match n {
            1 => WuXing::木,
            2 => WuXing::火,
            3 => WuXing::土,
            4 => WuXing::金,
            _ => WuXing::水,
        }
    }

    // 阳
    pub fn masculine(&self) -> bool {
        self.num() % 2 != 0
    }

    // 太玄数
    pub fn tai_xuan(&self) -> u8 {
        if self.num() <= 6 {
            10 - self.num()
        } else {
            16 - self.num()
        }
    }

    pub fn plus(&self, other: isize) -> Self {
        let n = if other < 0 {
            other - other / 12 * 12 + 12
        } else {
            other
        };

        let mut n = (self.num() as usize + n as usize) % 12;
        if n == 0 {
            n = 12;
        }
        match n {
            1 => DiZhi::子,
            2 => DiZhi::丑,
            3 => DiZhi::寅,
            4 => DiZhi::卯,
            5 => DiZhi::辰,
            6 => DiZhi::巳,
            7 => DiZhi::午,
            8 => DiZhi::未,
            9 => DiZhi::申,
            10 => DiZhi::酉,
            11 => DiZhi::戌,
            _ => DiZhi::亥,
        }
    }

    pub fn minus(&self, other: &DiZhi) -> u8 {
        // 返回整数值
        // self.num - other.num + 12
        (12 + self.num() - other.num()) % 12
    }

    pub fn ke(&self, t2: &DiZhi) -> bool {
        self.wu_xing().ke(&t2.wu_xing())
    }

    pub fn sheng(&self, t2: &DiZhi) -> bool {
        self.wu_xing().sheng(&t2.wu_xing())
    }

    ///  三合
    pub fn san_he(&self, other: &DiZhi) -> bool {
        if self == other {
            return false;
        }
        (self.num() as isize - other.num() as isize) % 4 == 0
    }

    /// 计算某一地支六合的公式：
    /// 六合地支 = 丑 - (地支 - 子)
    ///  六合
    pub fn liu_he(&self, other: &DiZhi) -> bool {
        if self == other {
            return false;
        }
        (self.num() as isize + other.num() as isize - 15) % 12 == 0
    }

    /// 六冲
    pub fn liu_chong(&self, other: &DiZhi) -> bool {
        if self == other {
            return false;
        }
        (self.num() as isize - other.num() as isize) % 6 == 0
    }

    /// 刑
    pub fn xing(&self, other: &DiZhi) -> bool {
        if *self == DiZhi::子 && *other == DiZhi::卯 {
            return true;
        }
        if *self == DiZhi::丑 && *other == DiZhi::戌 {
            return true;
        }
        if *self == DiZhi::寅 && *other == DiZhi::巳 {
            return true;
        }
        if *self == DiZhi::卯 && *other == DiZhi::子 {
            return true;
        }
        if *self == DiZhi::辰 && *other == DiZhi::辰 {
            return true;
        }
        if *self == DiZhi::巳 && *other == DiZhi::申 {
            return true;
        }
        if *self == DiZhi::午 && *other == DiZhi::午 {
            return true;
        }
        if *self == DiZhi::未 && *other == DiZhi::丑 {
            return true;
        }
        if *self == DiZhi::申 && *other == DiZhi::寅 {
            return true;
        }
        if *self == DiZhi::酉 && *other == DiZhi::酉 {
            return true;
        }
        if *self == DiZhi::戌 && *other == DiZhi::未 {
            return true;
        }
        if *self == DiZhi::亥 && *other == DiZhi::亥 {
            return true;
        }
        return false;
    }
}

impl Display for DiZhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiZhi::子 => write!(f, "{}", "子"),
            DiZhi::丑 => write!(f, "{}", "丑"),
            DiZhi::寅 => write!(f, "{}", "寅"),
            DiZhi::卯 => write!(f, "{}", "卯"),
            DiZhi::辰 => write!(f, "{}", "辰"),
            DiZhi::巳 => write!(f, "{}", "巳"),
            DiZhi::午 => write!(f, "{}", "午"),
            DiZhi::未 => write!(f, "{}", "未"),
            DiZhi::申 => write!(f, "{}", "申"),
            DiZhi::酉 => write!(f, "{}", "酉"),
            DiZhi::戌 => write!(f, "{}", "戌"),
            DiZhi::亥 => write!(f, "{}", "亥"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wuxing::WuXing;

    use super::DiZhi;

    const DI_ZHI: [DiZhi; 12] = [
        DiZhi::子,
        DiZhi::丑,
        DiZhi::寅,
        DiZhi::卯,
        DiZhi::辰,
        DiZhi::巳,
        DiZhi::午,
        DiZhi::未,
        DiZhi::申,
        DiZhi::酉,
        DiZhi::戌,
        DiZhi::亥,
    ];

    #[test]
    fn test_num() {
        assert_eq!(DiZhi::子.num(), 1);
        assert_eq!(DiZhi::丑.num(), 2);
        assert_eq!(DiZhi::寅.num(), 3);
        assert_eq!(DiZhi::卯.num(), 4);
        assert_eq!(DiZhi::辰.num(), 5);
        assert_eq!(DiZhi::巳.num(), 6);
        assert_eq!(DiZhi::午.num(), 7);
        assert_eq!(DiZhi::未.num(), 8);
        assert_eq!(DiZhi::申.num(), 9);
        assert_eq!(DiZhi::酉.num(), 10);
        assert_eq!(DiZhi::戌.num(), 11);
        assert_eq!(DiZhi::亥.num(), 12);
    }

    #[test]
    fn test_wu_xing() {
        assert_eq!(DiZhi::子.wu_xing(), WuXing::水);
        assert_eq!(DiZhi::丑.wu_xing(), WuXing::土);
        assert_eq!(DiZhi::寅.wu_xing(), WuXing::木);
        assert_eq!(DiZhi::卯.wu_xing(), WuXing::木);
        assert_eq!(DiZhi::辰.wu_xing(), WuXing::土);
        assert_eq!(DiZhi::巳.wu_xing(), WuXing::火);
        assert_eq!(DiZhi::午.wu_xing(), WuXing::火);
        assert_eq!(DiZhi::未.wu_xing(), WuXing::土);
        assert_eq!(DiZhi::申.wu_xing(), WuXing::金);
        assert_eq!(DiZhi::酉.wu_xing(), WuXing::金);
        assert_eq!(DiZhi::戌.wu_xing(), WuXing::土);
        assert_eq!(DiZhi::亥.wu_xing(), WuXing::水);
    }

    #[test]
    fn test_masculine() {
        assert!(DiZhi::子.masculine());
        assert!(DiZhi::寅.masculine());
        assert!(DiZhi::辰.masculine());
        assert!(DiZhi::午.masculine());
        assert!(DiZhi::申.masculine());
        assert!(DiZhi::戌.masculine());

        assert!(!DiZhi::丑.masculine());
        assert!(!DiZhi::卯.masculine());
        assert!(!DiZhi::巳.masculine());
        assert!(!DiZhi::未.masculine());
        assert!(!DiZhi::酉.masculine());
        assert!(!DiZhi::亥.masculine());
    }

    #[test]
    fn tai_xuan() {
        assert_eq!(DiZhi::子.tai_xuan(), 9);
        assert_eq!(DiZhi::午.tai_xuan(), 9);

        assert_eq!(DiZhi::丑.tai_xuan(), 8);
        assert_eq!(DiZhi::未.tai_xuan(), 8);

        assert_eq!(DiZhi::寅.tai_xuan(), 7);
        assert_eq!(DiZhi::申.tai_xuan(), 7);

        assert_eq!(DiZhi::卯.tai_xuan(), 6);
        assert_eq!(DiZhi::酉.tai_xuan(), 6);

        assert_eq!(DiZhi::辰.tai_xuan(), 5);
        assert_eq!(DiZhi::戌.tai_xuan(), 5);

        assert_eq!(DiZhi::巳.tai_xuan(), 4);
        assert_eq!(DiZhi::亥.tai_xuan(), 4);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DiZhi::子), "子");
        assert_eq!(format!("{}", DiZhi::丑), "丑");
        assert_eq!(format!("{}", DiZhi::寅), "寅");
        assert_eq!(format!("{}", DiZhi::卯), "卯");
        assert_eq!(format!("{}", DiZhi::辰), "辰");
        assert_eq!(format!("{}", DiZhi::巳), "巳");
        assert_eq!(format!("{}", DiZhi::午), "午");
        assert_eq!(format!("{}", DiZhi::未), "未");
        assert_eq!(format!("{}", DiZhi::申), "申");
        assert_eq!(format!("{}", DiZhi::酉), "酉");
        assert_eq!(format!("{}", DiZhi::戌), "戌");
        assert_eq!(format!("{}", DiZhi::亥), "亥");
    }

    // 地支相等
    #[test]
    fn test_di_zhi_equals() {
        assert_eq!(DiZhi::子, DiZhi::子);
        assert_eq!(DiZhi::丑, DiZhi::丑);
        assert_eq!(DiZhi::寅, DiZhi::寅);
        assert_eq!(DiZhi::卯, DiZhi::卯);
        assert_eq!(DiZhi::辰, DiZhi::辰);
        assert_eq!(DiZhi::巳, DiZhi::巳);
        assert_eq!(DiZhi::午, DiZhi::午);
        assert_eq!(DiZhi::未, DiZhi::未);
        assert_eq!(DiZhi::申, DiZhi::申);
        assert_eq!(DiZhi::酉, DiZhi::酉);
        assert_eq!(DiZhi::戌, DiZhi::戌);
        assert_eq!(DiZhi::亥, DiZhi::亥);
    }

    // 地支不相等
    #[test]
    fn test_di_zhi_not_eequal() {
        for (i, _) in DI_ZHI.iter().enumerate() {
            let g0 = DI_ZHI[i].clone();
            for j in 1..DI_ZHI.len() {
                let n = (i + j) % DI_ZHI.len();
                let g1 = DI_ZHI[n].clone();
                assert_ne!(g0, g1);
                assert_ne!(g1, g0);
            }
        }
    }

    // 地支 + 整数
    //用数学归纳法
    #[test]
    fn test_di_zhi_plus() {
        // 地支 + 1
        for (i, _) in DI_ZHI.iter().enumerate() {
            let tg0 = DI_ZHI[(i + 1) % DI_ZHI.len()].clone();
            let tg1 = DI_ZHI[i].clone();
            let tg1 = tg1.plus(1);
            assert_eq!(tg0, tg1);
        }

        // 地支 -1
        for (i, _) in DI_ZHI.iter().enumerate() {
            // i-1+diZhiName.len()
            let tg0 = DI_ZHI[(i + DI_ZHI.len() - 1) % DI_ZHI.len()].clone();
            let tg1 = DI_ZHI[i].clone();
            let tg1 = tg1.plus(-1);
            assert_eq!(tg0, tg1);
        }

        for g in DI_ZHI {
            assert_eq!(g.plus(99).plus(1), g.plus(100));
            assert_eq!(g.plus(-99).plus(-1), g.plus(-100));
        }
    }

    // 地支 - 地支 = 正整数
    #[test]
    fn test_di_zhi_minus() {
        for (i, _) in DI_ZHI.iter().enumerate() {
            let g0 = DI_ZHI[i].clone();
            for (j, _) in DI_ZHI.iter().enumerate() {
                let g1 = g0.plus(j as isize);
                assert_eq!(j, g1.minus(&g0) as usize);
            }
        }
    }

    // 地支相克
    #[test]
    fn test_di_zhi_ke() {
        for (i, _) in DI_ZHI.iter().enumerate() {
            let g0 = DI_ZHI[i].clone();
            for (j, _) in DI_ZHI.iter().enumerate() {
                let g1 = DI_ZHI[j].clone();
                assert_eq!(g0.wu_xing().ke(&g1.wu_xing()), g0.ke(&g1))
            }
        }
    }

    // 地支相生
    #[test]
    fn test_di_zhi_sheng() {
        for (i, _) in DI_ZHI.iter().enumerate() {
            let g0 = DI_ZHI[i].clone();
            for (j, _) in DI_ZHI.iter().enumerate() {
                let g1 = DI_ZHI[j].clone();
                assert_eq!(g0.wu_xing().sheng(&g1.wu_xing()), g0.sheng(&g1));
            }
        }
    }

    // 地支三合
    #[test]
    fn test_san_he() {
        for d0 in DI_ZHI {
            let d1 = d0.plus(4);
            let d2 = d1.plus(4);
            assert!(d0.san_he(&d1));
            assert!(d0.san_he(&d2));
            for d in DI_ZHI {
                if d != d1 && d != d2 {
                    assert!(!d0.san_he(&d));
                }
            }
        }
    }

    // 测试六合
    #[test]
    // 计算某一地支六合的公式：
    // 六合地支 = 丑 - (地支 - 子)
    fn test_liu_he() {
        for d0 in DI_ZHI {
            let n = d0.minus(&DiZhi::子);
            let d1 = DiZhi::丑.plus(n as isize * -1);
            assert!(d0.liu_he(&d1));

            for d in DI_ZHI {
                if d != d1 {
                    assert!(!d0.liu_he(&d));
                }
            }
        }
    }

    // 测试六冲
    #[test]
    fn test_liu_chong() {
        for d0 in DI_ZHI {
            let d1 = d0.plus(6);
            assert!(d0.liu_chong(&d1));
            for d in DI_ZHI {
                if d != d1 {
                    assert!(!d0.liu_chong(&d));
                }
            }
        }
    }

    #[test]
    fn test_xing() {
        let xing = [
            DiZhi::卯,
            DiZhi::戌,
            DiZhi::巳,
            DiZhi::子,
            DiZhi::辰,
            DiZhi::申,
            DiZhi::午,
            DiZhi::丑,
            DiZhi::寅,
            DiZhi::酉,
            DiZhi::未,
            DiZhi::亥,
        ];
        for (index, d0) in DI_ZHI.iter().enumerate() {
            let d1 = xing[index].clone();
            assert!(d0.xing(&d1));
            for d in DI_ZHI {
                if d != d1 {
                    assert!(!d0.xing(&d));
                }
            }
        }
    }

    #[test]
    fn test_default() {
        let d: DiZhi = Default::default();
        assert_eq!(d, DiZhi::子);
    }
}
