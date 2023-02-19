use std::fmt::Display;

use crate::wuxing::{WuXing, WU_XING_NUM_TO_NAME};

const DI_ZHI_NUM_TO_NAME: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];
#[derive(Debug, Eq, Clone)]
pub struct DiZhi {
    pub name: String,
    num: u8,
    pub wu_xing: WuXing,
    // 阳
    pub masculine: bool,
    // 太玄数
    pub tai_xuan: u8,
}

impl DiZhi {
    pub fn new(name: &str) -> Result<Self, String> {
        if let Some(num) = DI_ZHI_NUM_TO_NAME.iter().position(|&s| s == name) {
            let num = num as u8 + 1;

            let wu_xing = {
                let mut tmp = (num + 10) % 12;
                if tmp == 0 {
                    tmp = 12;
                }

                if tmp % 3 == 0 {
                    WuXing::new("土").unwrap()
                } else {
                    let tmp = tmp / 3 + tmp / 7 + 1;
                    WuXing::new(WU_XING_NUM_TO_NAME[tmp as usize - 1]).unwrap()
                }
            };

            Ok(Self {
                name: name.to_string(),
                num,
                wu_xing,
                masculine: num % 2 != 0,
                tai_xuan: if num <= 6 { 10 - num } else { 16 - num },
            })
        } else {
            Err(format!("没有此地支：{}", name))
        }
    }

    pub fn plus(&self, other: isize) -> Self {
        let tmp = if other < 0 {
            other - other / 12 * 12 + 12
        } else {
            other
        };

        let mut tmp = (self.num as usize + tmp as usize) % 12;
        if tmp == 0 {
            tmp = 12;
        }
        DiZhi::new(DI_ZHI_NUM_TO_NAME[tmp - 1]).unwrap()
    }

    pub fn minus(&self, other: &DiZhi) -> u8 {
        // 返回整数值
        // self.num - other.num + 12
        (12 + self.num - other.num) % 12
    }

    pub fn ke(&self, t2: &DiZhi) -> bool {
        self.wu_xing.ke(&t2.wu_xing)
    }

    pub fn sheng(&self, t2: &DiZhi) -> bool {
        self.wu_xing.sheng(&t2.wu_xing)
    }

    ///  三合
    pub fn san_he(&self, other: &DiZhi) -> bool {
        if self == other {
            return false;
        }
        (self.num as isize - other.num as isize) % 4 == 0
    }

    // 计算某一地支六合的公式：
    // 六合地支 = 丑 - (地支 - 子)
    ///  六合
    pub fn liu_he(&self, other: &DiZhi) -> bool {
        if self == other {
            return false;
        }
        (self.num as isize + other.num as isize - 15) % 12 == 0
    }

    /// 六冲
    pub fn liu_chong(&self, other: &DiZhi) -> bool {
        if self == other {
            return false;
        }
        (self.num as isize - other.num as isize) % 6 == 0
    }

    /// 刑
    pub fn xing(&self, other: &DiZhi) -> bool {
        if self.name == "子" && other.name == "卯" {
            return true;
        }
        if self.name == "丑" && other.name == "戌" {
            return true;
        }
        if self.name == "寅" && other.name == "巳" {
            return true;
        }
        if self.name == "卯" && other.name == "子" {
            return true;
        }
        if self.name == "辰" && other.name == "辰" {
            return true;
        }
        if self.name == "巳" && other.name == "申" {
            return true;
        }
        if self.name == "午" && other.name == "午" {
            return true;
        }
        if self.name == "未" && other.name == "丑" {
            return true;
        }
        if self.name == "申" && other.name == "寅" {
            return true;
        }
        if self.name == "酉" && other.name == "酉" {
            return true;
        }
        if self.name == "戌" && other.name == "未" {
            return true;
        }
        if self.name == "亥" && other.name == "亥" {
            return true;
        }
        return false;
    }
}

impl Display for DiZhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for DiZhi {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

#[cfg(test)]
mod tests {
    use crate::wuxing::WuXing;

    use super::DiZhi;

    const DI_ZHI_NAME: [&str; 12] = [
        "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
    ];

    // NewDiZhi()方法
    #[test]
    fn test_new() {
        let w = DiZhi::new("子").unwrap();
        assert_eq!(w.name, "子");
        assert_eq!(w.wu_xing, WuXing::new("水").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 9);

        let w = DiZhi::new("丑").unwrap();
        assert_eq!(w.name, "丑");
        assert_eq!(w.wu_xing, WuXing::new("土").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 8);

        let w = DiZhi::new("寅").unwrap();
        assert_eq!(w.name, "寅");
        assert_eq!(w.wu_xing, WuXing::new("木").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 7);

        let w = DiZhi::new("卯").unwrap();
        assert_eq!(w.name, "卯");
        assert_eq!(w.wu_xing, WuXing::new("木").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 6);

        let w = DiZhi::new("辰").unwrap();
        assert_eq!(w.name, "辰");
        assert_eq!(w.wu_xing, WuXing::new("土").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 5);

        let w = DiZhi::new("巳").unwrap();
        assert_eq!(w.name, "巳");
        assert_eq!(w.wu_xing, WuXing::new("火").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 4);

        let w = DiZhi::new("午").unwrap();
        assert_eq!(w.name, "午");
        assert_eq!(w.wu_xing, WuXing::new("火").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 9);

        let w = DiZhi::new("未").unwrap();
        assert_eq!(w.name, "未");
        assert_eq!(w.wu_xing, WuXing::new("土").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 8);

        let w = DiZhi::new("申").unwrap();
        assert_eq!(w.name, "申");
        assert_eq!(w.wu_xing, WuXing::new("金").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 7);

        let w = DiZhi::new("酉").unwrap();
        assert_eq!(w.name, "酉");
        assert_eq!(w.wu_xing, WuXing::new("金").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 6);

        let w = DiZhi::new("戌").unwrap();
        assert_eq!(w.name, "戌");
        assert_eq!(w.wu_xing, WuXing::new("土").unwrap());
        assert!(w.masculine);
        assert_eq!(w.tai_xuan, 5);

        let w = DiZhi::new("亥").unwrap();
        assert_eq!(w.name, "亥");
        assert_eq!(w.wu_xing, WuXing::new("水").unwrap());
        assert!(!w.masculine);
        assert_eq!(w.tai_xuan, 4);

        // 不正确地支创建
        assert!(DiZhi::new("否").is_err());
    }

    #[test]
    fn test_display() {
        for value in DI_ZHI_NAME {
            let w = DiZhi::new(value).unwrap();
            let w = format!("{}", w);
            assert_eq!(value, w);
        }
    }

    // 地支相等
    #[test]
    fn test_di_zhi_equals() {
        for name in DI_ZHI_NAME {
            let tg0 = DiZhi::new(name).unwrap();
            let tg1 = DiZhi::new(name).unwrap();
            assert_eq!(tg0, tg1);
            assert_eq!(tg1, tg0);
        }
    }

    // 地支不相等
    #[test]
    fn test_di_zhi_not_eequal() {
        for (i, _) in DI_ZHI_NAME.iter().enumerate() {
            let g0 = DiZhi::new(DI_ZHI_NAME[i]).unwrap();
            for j in 1..DI_ZHI_NAME.len() {
                let n = (i + j) % DI_ZHI_NAME.len();
                let g1 = DiZhi::new(DI_ZHI_NAME[n]).unwrap();
                assert_ne!(g0, g1);
                assert_ne!(g1, g0);
            }
        }
    }

    // 地支 + 整数
    //用数学归纳法
    #[test]
    fn test_di_zhi_plus() {
        for (i, _) in DI_ZHI_NAME.iter().enumerate() {
            let tg0 = DiZhi::new(DI_ZHI_NAME[(i + 1) % DI_ZHI_NAME.len()]).unwrap();
            let tg1 = DiZhi::new(DI_ZHI_NAME[i]).unwrap();
            let tg1 = tg1.plus(1);
            assert_eq!(tg0, tg1);
        }

        for (i, _) in DI_ZHI_NAME.iter().enumerate() {
            // i-1+diZhiName.len()
            let tg0 =
                DiZhi::new(DI_ZHI_NAME[(i + DI_ZHI_NAME.len() - 1) % DI_ZHI_NAME.len()]).unwrap();
            let tg1 = DiZhi::new(DI_ZHI_NAME[i]).unwrap();
            let tg1 = tg1.plus(-1);
            assert_eq!(tg0, tg1);
        }

        for it in DI_ZHI_NAME {
            let g = DiZhi::new(it).unwrap();
            assert_eq!(g.plus(99).plus(1), g.plus(100));
            assert_eq!(g.plus(-99).plus(-1), g.plus(-100));
        }
    }
    // 地支 - 地支 = 正整数
    #[test]
    fn test_di_zhi_minus() {
        for (i, _) in DI_ZHI_NAME.iter().enumerate() {
            let g0 = DiZhi::new(DI_ZHI_NAME[i]).unwrap();
            for (j, _) in DI_ZHI_NAME.iter().enumerate() {
                let g1 = g0.plus(j as isize);
                assert_eq!(j, g1.minus(&g0) as usize);
            }
        }
    }

    // 地支相克
    #[test]
    fn test_di_zhi_ke() {
        for (i, _) in DI_ZHI_NAME.iter().enumerate() {
            let g0 = DiZhi::new(DI_ZHI_NAME[i]).unwrap();
            for (j, _) in DI_ZHI_NAME.iter().enumerate() {
                let g1 = DiZhi::new(DI_ZHI_NAME[j]).unwrap();
                assert_eq!(g0.wu_xing.ke(&g1.wu_xing), g0.ke(&g1))
            }
        }
    }
    // 地支相生
    #[test]
    fn test_di_zhi_sheng() {
        for (i, _) in DI_ZHI_NAME.iter().enumerate() {
            let g0 = DiZhi::new(DI_ZHI_NAME[i]).unwrap();
            for (j, _) in DI_ZHI_NAME.iter().enumerate() {
                let g1 = DiZhi::new(DI_ZHI_NAME[j]).unwrap();
                assert_eq!(g0.wu_xing.sheng(&g1.wu_xing), g0.sheng(&g1));
            }
        }
    }

    // 地支三合
    #[test]
    fn test_san_he() {
        for it in DI_ZHI_NAME {
            let d0 = DiZhi::new(it).unwrap();
            let d1 = d0.plus(4);
            let d2 = d1.plus(4);
            assert!(d0.san_he(&d1));
            assert!(d0.san_he(&d2));
            for i in DI_ZHI_NAME {
                let d = DiZhi::new(i).unwrap();
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
        let zi = DiZhi::new("子").unwrap();
        let chou = DiZhi::new("丑").unwrap();
        for name in DI_ZHI_NAME {
            let d0 = DiZhi::new(name).unwrap();
            let n = d0.minus(&zi);
            let d1 = chou.plus(n as isize * -1);
            assert!(d0.liu_he(&d1));

            for j in DI_ZHI_NAME {
                let d = DiZhi::new(j).unwrap();
                if d != d1 {
                    assert!(!d0.liu_he(&d));
                }
            }
        }
    }

    // 测试六冲
    #[test]
    fn test_liu_chong() {
        for it in DI_ZHI_NAME {
            let d0 = DiZhi::new(it).unwrap();
            let d1 = d0.plus(6);
            assert!(d0.liu_chong(&d1));
            for i in DI_ZHI_NAME {
                let d = DiZhi::new(i).unwrap();
                if d != d1 {
                    assert!(!d0.liu_chong(&d));
                }
            }
        }
    }

    #[test]
    fn test_xing() {
        let xing = [
            "卯", "戌", "巳", "子", "辰", "申", "午", "丑", "寅", "酉", "未", "亥",
        ];
        for (index, name) in DI_ZHI_NAME.iter().enumerate() {
            let d0 = DiZhi::new(name).unwrap();
            let d1 = DiZhi::new(xing[index]).unwrap();
            assert!(d0.xing(&d1));
            for j in DI_ZHI_NAME {
                let d = DiZhi::new(j).unwrap();
                if d != d1 {
                    assert!(!d0.xing(&d));
                }
            }
        }
    }
}
