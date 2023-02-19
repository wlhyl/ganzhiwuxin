use std::fmt::Display;

use crate::wuxing::{WuXing, WU_XING_NUM_TO_NAME};

const TIAN_GAN_NUM_TO_NAME: [&str; 10] =
    ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

#[derive(Debug, Eq, Clone)]
pub struct TianGan {
    pub name: String,
    num: u8,
    pub wu_xing: WuXing,
    //  阳
    pub masculine: bool,
    // 太玄数
    pub tai_xuan: u8,
}

impl TianGan {
    pub fn new(name: &str) -> Result<Self, String> {
        if let Some(num) = TIAN_GAN_NUM_TO_NAME.iter().position(|&s| s == name) {
            let num = num as u8 + 1;
            let wu_xing_num = (num + 1) / 2;

            Ok(Self {
                name: name.to_string(),
                num,
                wu_xing: WuXing {
                    name: WU_XING_NUM_TO_NAME[wu_xing_num as usize - 1].to_owned(),
                    num: wu_xing_num,
                },
                masculine: num % 2 != 0,
                tai_xuan: if num <= 5 { 10 - num } else { 15 - num },
            })
        } else {
            Err(format!("没有此天干：{}", name))
        }
    }

    /// 天干向前数n个
    pub fn plus(&self, other: isize) -> Self {
        let tmp = if other < 0 {
            other - other / 10 * 10 + 10
        } else {
            other
        };

        let mut tmp = (self.num as usize + tmp as usize) % 10;
        if tmp == 0 {
            tmp = 10
        }
        TianGan::new(TIAN_GAN_NUM_TO_NAME[tmp - 1]).unwrap()
    }

    /// 两天干相差几位
    pub fn minus(&self, other: &TianGan) -> u8 {
        // 返回值为整数
        //  (self.num - other.num + 10) % 10
        // 1-2会溢出
        return (10 + self.num - other.num) % 10;
    }

    ///  克
    pub fn ke(&self, other: &TianGan) -> bool {
        self.wu_xing.ke(&other.wu_xing)
    }

    ///  生
    pub fn sheng(&self, other: &TianGan) -> bool {
        self.wu_xing.sheng(&other.wu_xing)
    }

    /// 五合
    pub fn wu_he(&self, other: &TianGan) -> bool {
        if self.num == other.num {
            false
        } else {
            return (self.num as isize - other.num as isize) % 5 == 0;
        }
    }
}

impl Display for TianGan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for TianGan {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

#[cfg(test)]
mod tests {
    use crate::wuxing::WuXing;

    use super::TianGan;

    const TIAN_GAN_NAME: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

    #[test]
    fn test_new() {
        let w = TianGan::new("甲").unwrap();
        assert_eq!(w.name, "甲");
        assert_eq!(w.wu_xing, WuXing::new("木").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 9);

        let w = TianGan::new("乙").unwrap();
        assert_eq!(w.name, "乙");
        assert_eq!(w.wu_xing, WuXing::new("木").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 8);

        let w = TianGan::new("丙").unwrap();
        assert_eq!(w.name, "丙");
        assert_eq!(w.wu_xing, WuXing::new("火").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 7);

        let w = TianGan::new("丁").unwrap();
        assert_eq!(w.name, "丁");
        assert_eq!(w.wu_xing, WuXing::new("火").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 6);

        let w = TianGan::new("戊").unwrap();
        assert_eq!(w.name, "戊");
        assert_eq!(w.wu_xing, WuXing::new("土").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 5);

        let w = TianGan::new("己").unwrap();
        assert_eq!(w.name, "己");
        assert_eq!(w.wu_xing, WuXing::new("土").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 9);

        let w = TianGan::new("庚").unwrap();
        assert_eq!(w.name, "庚");
        assert_eq!(w.wu_xing, WuXing::new("金").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 8);

        let w = TianGan::new("辛").unwrap();
        assert_eq!(w.name, "辛");
        assert_eq!(w.wu_xing, WuXing::new("金").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 7);

        let w = TianGan::new("壬").unwrap();
        assert_eq!(w.name, "壬");
        assert_eq!(w.wu_xing, WuXing::new("水").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 6);

        let w = TianGan::new("癸").unwrap();
        assert_eq!(w.name, "癸");
        assert_eq!(w.wu_xing, WuXing::new("水").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 5);

        let w = TianGan::new("否");
        assert!(w.is_err());
    }

    #[test]
    fn test_display() {
        for value in TIAN_GAN_NAME {
            let w = TianGan::new(value).unwrap();
            let w = format!("{}", w);
            assert_eq!(value, w);
        }
    }

    #[test]
    fn test_tian_gan_equals() {
        for name in TIAN_GAN_NAME {
            let w0 = TianGan::new(name).unwrap();
            let w1 = TianGan::new(name).unwrap();
            assert_eq!(w0, w1);
            assert_eq!(w1, w0);
        }
    }

    // 天干不相等
    #[test]
    fn test_tian_gan_mot_equal() {
        for (i, _) in TIAN_GAN_NAME.iter().enumerate() {
            let g0 = TianGan::new(TIAN_GAN_NAME[i]).unwrap();
            for j in 1..TIAN_GAN_NAME.len() {
                let n = (i + j) % TIAN_GAN_NAME.len();
                let g1 = TianGan::new(TIAN_GAN_NAME[n]).unwrap();

                assert_ne!(g0, g1);
                assert_ne!(g1, g0);
            }
        }
    }

    //用数学归纳法
    #[test]
    fn test_tian_gan_plus() {
        for (i, _) in TIAN_GAN_NAME.iter().enumerate() {
            let tg0 = TianGan::new(TIAN_GAN_NAME[(i + 1) % TIAN_GAN_NAME.len()]).unwrap();
            let tg1 = TianGan::new(TIAN_GAN_NAME[i]).unwrap();
            let tg1 = tg1.plus(1);
            assert_eq!(tg0, tg1);
        }

        // plus(-1)
        for (i, _) in TIAN_GAN_NAME.iter().enumerate() {
            // i+(-1)+TIAN_GAN_NAME.len(), usize 0-1会溢出
            let n = (TIAN_GAN_NAME.len() + i - 1) % TIAN_GAN_NAME.len();
            let tg0 = TianGan::new(TIAN_GAN_NAME[n]).unwrap();
            let tg1 = TianGan::new(TIAN_GAN_NAME[i]).unwrap();
            let tg1 = tg1.plus(-1);
            assert_eq!(tg0, tg1);
        }

        for it in TIAN_GAN_NAME {
            let g = TianGan::new(it).unwrap();
            assert_eq!(g.plus(99).plus(1), g.plus(100));
            assert_eq!(g.plus(-99).plus(-1), g.plus(-100));
        }
    }
    // "天干 - 天干 = 正整数"
    #[test]
    fn test_tian_gan_minus() {
        for (i, _) in TIAN_GAN_NAME.iter().enumerate() {
            let g0 = TianGan::new(TIAN_GAN_NAME[i]).unwrap();
            for (j, _) in TIAN_GAN_NAME.iter().enumerate() {
                let g1 = g0.plus(j as isize);
                assert_eq!(j, g1.minus(&g0) as usize);
            }
        }
    }

    // 天干相克
    #[test]
    fn test_tian_gan_ke() {
        for (i, _) in TIAN_GAN_NAME.iter().enumerate() {
            let g0 = TianGan::new(TIAN_GAN_NAME[i]).unwrap();
            for (j, _) in TIAN_GAN_NAME.iter().enumerate() {
                let g1 = TianGan::new(TIAN_GAN_NAME[j]).unwrap();
                assert_eq!(g0.wu_xing.ke(&g1.wu_xing), g0.ke(&g1));
            }
        }
    }

    // 天干相生
    #[test]
    fn test_tian_gan_sheng() {
        for (i, _) in TIAN_GAN_NAME.iter().enumerate() {
            let g0 = TianGan::new(TIAN_GAN_NAME[i]).unwrap();
            for (j, _) in TIAN_GAN_NAME.iter().enumerate() {
                let g1 = TianGan::new(TIAN_GAN_NAME[j]).unwrap();
                assert_eq!(g0.wu_xing.sheng(&g1.wu_xing), g0.sheng(&g1));
            }
        }
    }

    // 天干五合
    #[test]
    fn test_wu_he() {
        for it in TIAN_GAN_NAME {
            let g0 = TianGan::new(it).unwrap();
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
}
