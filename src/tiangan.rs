use std::fmt::Display;

use crate::wuxing::WuXing;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum TianGan {
    #[default]
    甲,
    乙,
    丙,
    丁,
    戊,
    己,
    庚,
    辛,
    壬,
    癸,
}
// pub struct TianGan {
//     pub name: String,
//     num: u8,
//     pub wu_xing: WuXing,
//     //  阳
//     pub masculine: bool,
//     // 太玄数
//     pub tai_xuan: u8,
// }

impl TianGan {
    fn num(&self) -> u8 {
        match self {
            TianGan::甲 => 1,
            TianGan::乙 => 2,
            TianGan::丙 => 3,
            TianGan::丁 => 4,
            TianGan::戊 => 5,
            TianGan::己 => 6,
            TianGan::庚 => 7,
            TianGan::辛 => 8,
            TianGan::壬 => 9,
            TianGan::癸 => 10,
        }
    }

    /// 五行
    pub fn wu_xing(&self) -> WuXing {
        let n = (self.num() + 1) / 2;
        match n {
            1 => WuXing::木,
            2 => WuXing::火,
            3 => WuXing::土,
            4 => WuXing::金,
            _ => WuXing::水,
        }
    }

    ///  阳
    pub fn masculine(&self) -> bool {
        self.num() % 2 != 0
    }

    /// 太玄数
    pub fn tai_xuan(&self) -> u8 {
        if self.num() <= 5 {
            10 - self.num()
        } else {
            15 - self.num()
        }
    }

    /// 天干向前数n个
    pub fn plus(&self, other: isize) -> Self {
        let tmp = if other < 0 {
            other - other / 10 * 10 + 10
        } else {
            other
        };

        let mut tmp = (self.num() as usize + tmp as usize) % 10;
        if tmp == 0 {
            tmp = 10
        }
        match tmp {
            1 => TianGan::甲,
            2 => TianGan::乙,
            3 => TianGan::丙,
            4 => TianGan::丁,
            5 => TianGan::戊,
            6 => TianGan::己,
            7 => TianGan::庚,
            8 => TianGan::辛,
            9 => TianGan::壬,
            _ => TianGan::癸,
        }
    }

    /// 两天干相差几位
    pub fn minus(&self, other: &TianGan) -> u8 {
        // 返回值为整数
        //  (self.num - other.num + 10) % 10
        // 1-2会溢出
        return (10 + self.num() - other.num()) % 10;
    }

    ///  克
    pub fn ke(&self, other: &TianGan) -> bool {
        self.wu_xing().ke(&other.wu_xing())
    }

    ///  生
    pub fn sheng(&self, other: &TianGan) -> bool {
        self.wu_xing().sheng(&other.wu_xing())
    }

    /// 五合
    pub fn wu_he(&self, other: &TianGan) -> bool {
        if self.num() == other.num() {
            false
        } else {
            return (self.num() as isize - other.num() as isize) % 5 == 0;
        }
    }
}

impl Display for TianGan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TianGan::甲 => write!(f, "{}", "甲"),
            TianGan::乙 => write!(f, "{}", "乙"),
            TianGan::丙 => write!(f, "{}", "丙"),
            TianGan::丁 => write!(f, "{}", "丁"),
            TianGan::戊 => write!(f, "{}", "戊"),
            TianGan::己 => write!(f, "{}", "己"),
            TianGan::庚 => write!(f, "{}", "庚"),
            TianGan::辛 => write!(f, "{}", "辛"),
            TianGan::壬 => write!(f, "{}", "壬"),
            TianGan::癸 => write!(f, "{}", "癸"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wuxing::WuXing;

    use super::TianGan;

    const TIAN_GAN: [TianGan; 10] = [
        TianGan::甲,
        TianGan::乙,
        TianGan::丙,
        TianGan::丁,
        TianGan::戊,
        TianGan::己,
        TianGan::庚,
        TianGan::辛,
        TianGan::壬,
        TianGan::癸,
    ];

    #[test]
    fn test_num() {
        assert_eq!(TianGan::甲.num(), 1);
        assert_eq!(TianGan::乙.num(), 2);
        assert_eq!(TianGan::丙.num(), 3);
        assert_eq!(TianGan::丁.num(), 4);
        assert_eq!(TianGan::戊.num(), 5);
        assert_eq!(TianGan::己.num(), 6);
        assert_eq!(TianGan::庚.num(), 7);
        assert_eq!(TianGan::辛.num(), 8);
        assert_eq!(TianGan::壬.num(), 9);
        assert_eq!(TianGan::癸.num(), 10);
    }
    #[test]
    fn test_wu_xing() {
        assert_eq!(TianGan::甲.wu_xing(), WuXing::木);
        assert_eq!(TianGan::乙.wu_xing(), WuXing::木);
        assert_eq!(TianGan::丙.wu_xing(), WuXing::火);
        assert_eq!(TianGan::丁.wu_xing(), WuXing::火);
        assert_eq!(TianGan::戊.wu_xing(), WuXing::土);
        assert_eq!(TianGan::己.wu_xing(), WuXing::土);
        assert_eq!(TianGan::庚.wu_xing(), WuXing::金);
        assert_eq!(TianGan::辛.wu_xing(), WuXing::金);
        assert_eq!(TianGan::壬.wu_xing(), WuXing::水);
        assert_eq!(TianGan::癸.wu_xing(), WuXing::水);
    }

    #[test]
    fn test_masculine() {
        assert!(TianGan::甲.masculine());
        assert!(TianGan::丙.masculine());
        assert!(TianGan::戊.masculine());
        assert!(TianGan::庚.masculine());
        assert!(TianGan::壬.masculine());

        assert!(!TianGan::乙.masculine());
        assert!(!TianGan::丁.masculine());
        assert!(!TianGan::己.masculine());
        assert!(!TianGan::辛.masculine());
        assert!(!TianGan::癸.masculine());
    }

    #[test]
    fn test_tai_xuan() {
        assert_eq!(TianGan::甲.tai_xuan(), 9);
        assert_eq!(TianGan::己.tai_xuan(), 9);

        assert_eq!(TianGan::乙.tai_xuan(), 8);
        assert_eq!(TianGan::庚.tai_xuan(), 8);

        assert_eq!(TianGan::丙.tai_xuan(), 7);
        assert_eq!(TianGan::辛.tai_xuan(), 7);

        assert_eq!(TianGan::戊.tai_xuan(), 5);
        assert_eq!(TianGan::癸.tai_xuan(), 5);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TianGan::甲), "甲");
        assert_eq!(format!("{}", TianGan::乙), "乙");
        assert_eq!(format!("{}", TianGan::丙), "丙");
        assert_eq!(format!("{}", TianGan::丁), "丁");
        assert_eq!(format!("{}", TianGan::戊), "戊");
        assert_eq!(format!("{}", TianGan::己), "己");
        assert_eq!(format!("{}", TianGan::庚), "庚");
        assert_eq!(format!("{}", TianGan::辛), "辛");
        assert_eq!(format!("{}", TianGan::壬), "壬");
        assert_eq!(format!("{}", TianGan::癸), "癸");
    }

    #[test]
    fn test_tian_gan_equals() {
        assert_eq!(TianGan::甲, TianGan::甲);
        assert_eq!(TianGan::乙, TianGan::乙);
        assert_eq!(TianGan::丙, TianGan::丙);
        assert_eq!(TianGan::丁, TianGan::丁);
        assert_eq!(TianGan::戊, TianGan::戊);
        assert_eq!(TianGan::己, TianGan::己);
        assert_eq!(TianGan::庚, TianGan::庚);
        assert_eq!(TianGan::辛, TianGan::辛);
        assert_eq!(TianGan::壬, TianGan::壬);
        assert_eq!(TianGan::癸, TianGan::癸);
    }

    // 天干不相等
    #[test]
    fn test_tian_gan_mot_equal() {
        for (i, _) in TIAN_GAN.iter().enumerate() {
            let g0 = TIAN_GAN[i].clone();
            for j in 1..TIAN_GAN.len() {
                let n = (i + j) % TIAN_GAN.len();
                let g1 = TIAN_GAN[n].clone();

                assert_ne!(g0, g1);
                assert_ne!(g1, g0);
            }
        }
    }

    //用数学归纳法
    #[test]
    fn test_tian_gan_plus() {
        // 天干+1
        for (i, _) in TIAN_GAN.iter().enumerate() {
            let tg0 = TIAN_GAN[(i + 1) % TIAN_GAN.len()].clone();
            let tg1 = TIAN_GAN[i].clone();
            let tg1 = tg1.plus(1);
            assert_eq!(tg0, tg1);
        }

        // 天干 -1
        for (i, _) in TIAN_GAN.iter().enumerate() {
            // i+(-1)+TIAN_GAN_NAME.len(), usize 0-1会溢出
            let n = (TIAN_GAN.len() + i - 1) % TIAN_GAN.len();
            let tg0 = TIAN_GAN[n].clone();
            let tg1 = TIAN_GAN[i].clone();
            let tg1 = tg1.plus(-1);
            assert_eq!(tg0, tg1);
        }

        for g in TIAN_GAN {
            assert_eq!(g.plus(99).plus(1), g.plus(100));
            assert_eq!(g.plus(-99).plus(-1), g.plus(-100));
        }
    }
    // "天干 - 天干 = 正整数"
    #[test]
    fn test_tian_gan_minus() {
        for (i, _) in TIAN_GAN.iter().enumerate() {
            let g0 = TIAN_GAN[i].clone();
            for (j, _) in TIAN_GAN.iter().enumerate() {
                let g1 = g0.plus(j as isize);
                assert_eq!(j, g1.minus(&g0) as usize);
            }
        }
    }

    // 天干相克
    #[test]
    fn test_tian_gan_ke() {
        for (i, _) in TIAN_GAN.iter().enumerate() {
            let g0 = TIAN_GAN[i].clone();
            for (j, _) in TIAN_GAN.iter().enumerate() {
                let g1 = TIAN_GAN[j].clone();
                assert_eq!(g0.wu_xing().ke(&g1.wu_xing()), g0.ke(&g1));
            }
        }
    }

    // 天干相生
    #[test]
    fn test_tian_gan_sheng() {
        for (i, _) in TIAN_GAN.iter().enumerate() {
            let g0 = TIAN_GAN[i].clone();
            for (j, _) in TIAN_GAN.iter().enumerate() {
                let g1 = TIAN_GAN[j].clone();
                assert_eq!(g0.wu_xing().sheng(&g1.wu_xing()), g0.sheng(&g1));
            }
        }
    }

    // 天干五合
    #[test]
    fn test_wu_he() {
        for g0 in TIAN_GAN {
            for i in 0..10 {
                let g1 = g0.plus(i);
                if i == 5 {
                    assert!(g0.wu_he(&g1));
                } else {
                    assert!(!g0.wu_he(&g1));
                }
            }
        }
    }

    #[test]
    fn test_default() {
        let g = TianGan::default();
        assert_eq!(g, TianGan::甲);
    }
}
