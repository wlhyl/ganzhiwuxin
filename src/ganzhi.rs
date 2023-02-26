use std::fmt::Display;

use crate::{dizhi::DiZhi, tiangan::TianGan, wuxing::WuXing};

#[derive(Clone, Debug)]
pub struct GanZhi {
    pub gan: TianGan,
    pub zhi: DiZhi,
    // 纳音
    pub na_yin: WuXing,
}

impl GanZhi {
    pub fn new(gan: TianGan, zhi: DiZhi) -> Result<Self, String> {
        if gan.masculine != zhi.masculine {
            return Err("干支阴阳不相同".to_owned());
        }

        let wu_xing_name = ["木", "火", "土", "金", "水"];
        let g = if gan.masculine {
            gan.plus(1)
        } else {
            gan.plus(-1)
        };
        let z = if zhi.masculine {
            zhi.plus(1)
        } else {
            zhi.plus(-1)
        };
        let n = (49 - (gan.tai_xuan + zhi.tai_xuan + g.tai_xuan + z.tai_xuan)) % 10;
        let w = if n == 1 || n == 6 {
            WuXing::new("水").unwrap()
        } else if n == 2 || n == 7 {
            WuXing::new("火").unwrap()
        } else if n == 3 || n == 8 {
            WuXing::new("木").unwrap()
        } else if n == 4 || n == 9 {
            WuXing::new("金").unwrap()
        } else {
            WuXing::new("土").unwrap()
        };

        // w所生者即为所求xxmw间
        // 依序为w的下一个
        // n = (wuXingName.indexOf(w.toString()) + 1) % 5
        let n = w.num % 5;

        Ok(Self {
            gan,
            zhi,
            na_yin: WuXing {
                name: wu_xing_name[n as usize].to_owned(),
                num: n + 1,
            },
        })
    }

    pub fn plus(&self, other: isize) -> Self {
        let g = self.gan.plus(other);
        let z = self.zhi.plus(other);
        GanZhi::new(g, z).unwrap()
    }

    pub fn minus(&self, other: &GanZhi) -> u8 {
        // 返回整数值
        let n = (self.num() - other.num() + 60) % 60;
        n.try_into().unwrap()
    }

    fn num(&self) -> isize {
        let g = TianGan::new("甲").unwrap();
        let z = DiZhi::new("子").unwrap();
        let mut n = 0;
        for i in 0..60 {
            if g.plus(i) == self.gan && z.plus(i) == self.zhi {
                n = i;
                break;
            }
        }
        n + 1
    }
}

impl Display for GanZhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.gan.name, self.zhi.name)
    }
}

impl PartialEq for GanZhi {
    fn eq(&self, other: &Self) -> bool {
        self.gan == other.gan && self.zhi == other.zhi
    }
}

impl Default for GanZhi {
    fn default() -> Self {
        Self::new(Default::default(), Default::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{dizhi::DiZhi, tiangan::TianGan, wuxing::WuXing};

    use super::GanZhi;

    const GAN_ZHI_NAME: [&str; 60] = [
        "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌",
        "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉",
        "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未", "丙申",
        "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯", "甲辰", "乙巳", "丙午", "丁未",
        "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午",
        "己未", "庚申", "辛酉", "壬戌", "癸亥",
    ];

    #[test]
    fn test_new() {
        let na_ying: HashMap<&str, WuXing> = HashMap::from([
            ("甲子", WuXing::new("金").unwrap()),
            ("乙丑", WuXing::new("金").unwrap()),
            ("丙寅", WuXing::new("火").unwrap()),
            ("丁卯", WuXing::new("火").unwrap()),
            ("戊辰", WuXing::new("木").unwrap()),
            ("己巳", WuXing::new("木").unwrap()),
            ("庚午", WuXing::new("土").unwrap()),
            ("辛未", WuXing::new("土").unwrap()),
            ("壬申", WuXing::new("金").unwrap()),
            ("癸酉", WuXing::new("金").unwrap()),
            ("甲戌", WuXing::new("火").unwrap()),
            ("乙亥", WuXing::new("火").unwrap()),
            ("丙子", WuXing::new("水").unwrap()),
            ("丁丑", WuXing::new("水").unwrap()),
            ("戊寅", WuXing::new("土").unwrap()),
            ("己卯", WuXing::new("土").unwrap()),
            ("庚辰", WuXing::new("金").unwrap()),
            ("辛巳", WuXing::new("金").unwrap()),
            ("壬午", WuXing::new("木").unwrap()),
            ("癸未", WuXing::new("木").unwrap()),
            ("甲申", WuXing::new("水").unwrap()),
            ("乙酉", WuXing::new("水").unwrap()),
            ("丙戌", WuXing::new("土").unwrap()),
            ("丁亥", WuXing::new("土").unwrap()),
            ("戊子", WuXing::new("火").unwrap()),
            ("己丑", WuXing::new("火").unwrap()),
            ("庚寅", WuXing::new("木").unwrap()),
            ("辛卯", WuXing::new("木").unwrap()),
            ("壬辰", WuXing::new("水").unwrap()),
            ("癸巳", WuXing::new("水").unwrap()),
            ("甲午", WuXing::new("金").unwrap()),
            ("乙未", WuXing::new("金").unwrap()),
            ("丙申", WuXing::new("火").unwrap()),
            ("丁酉", WuXing::new("火").unwrap()),
            ("戊戌", WuXing::new("木").unwrap()),
            ("己亥", WuXing::new("木").unwrap()),
            ("庚子", WuXing::new("土").unwrap()),
            ("辛丑", WuXing::new("土").unwrap()),
            ("壬寅", WuXing::new("金").unwrap()),
            ("癸卯", WuXing::new("金").unwrap()),
            ("甲辰", WuXing::new("火").unwrap()),
            ("乙巳", WuXing::new("火").unwrap()),
            ("丙午", WuXing::new("水").unwrap()),
            ("丁未", WuXing::new("水").unwrap()),
            ("戊申", WuXing::new("土").unwrap()),
            ("己酉", WuXing::new("土").unwrap()),
            ("庚戌", WuXing::new("金").unwrap()),
            ("辛亥", WuXing::new("金").unwrap()),
            ("壬子", WuXing::new("木").unwrap()),
            ("癸丑", WuXing::new("木").unwrap()),
            ("甲寅", WuXing::new("水").unwrap()),
            ("乙卯", WuXing::new("水").unwrap()),
            ("丙辰", WuXing::new("土").unwrap()),
            ("丁巳", WuXing::new("土").unwrap()),
            ("戊午", WuXing::new("火").unwrap()),
            ("己未", WuXing::new("火").unwrap()),
            ("庚申", WuXing::new("木").unwrap()),
            ("辛酉", WuXing::new("木").unwrap()),
            ("壬戌", WuXing::new("水").unwrap()),
            ("癸亥", WuXing::new("水").unwrap()),
        ]);

        let jia = TianGan::new("甲").unwrap();
        let zi = DiZhi::new("子").unwrap();
        for i in 0..10 {
            let g = jia.plus(i);
            for j in 0..12 {
                let z = zi.plus(j);
                let gz = GanZhi::new(g.clone(), z.clone());
                if g.masculine == z.masculine {
                    assert!(gz.is_ok(), "new方法错误, {}{}是正确的干支", g.name, z.name);
                    let gz = gz.unwrap();
                    assert_eq!(gz.gan, g);
                    assert_eq!(gz.zhi, z);
                    let name: &str = &format!("{}", gz);
                    assert_eq!(name, format!("{}{}", g.name, z.name));

                    assert_eq!(gz.na_yin, na_ying.get(name).unwrap().clone());
                } else {
                    assert!(
                        gz.is_err(),
                        "new()方法错误, {}{}不是正确的干支",
                        g.name,
                        z.name
                    );
                }
            }
        }
    }

    // 干支相等
    #[test]
    fn test_gan_zhi_equals() {
        for name in GAN_ZHI_NAME {
            let mut gz = name.chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();
            let g = TianGan::new(&g).unwrap();
            let z = DiZhi::new(&z).unwrap();
            let gz0 = GanZhi::new(g.clone(), z.clone()).unwrap();
            let gz1 = GanZhi::new(g, z).unwrap();

            assert_eq!(gz0, gz1, "{} 与 {} 相等", gz0, gz1);
        }
    }
    // 干支不相等
    #[test]
    fn test_gan_zhi_not_equals() {
        for (i, name) in GAN_ZHI_NAME.iter().enumerate() {
            let mut gz = name.chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();
            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz0 = GanZhi::new(g0, z0).unwrap();

            for j in 1..60 {
                let n = (i + j) % 60;
                let mut gz = GAN_ZHI_NAME[n].chars();
                let g = gz.next().unwrap().to_string();
                let z = gz.next().unwrap().to_string();
                let g1 = TianGan::new(&g).unwrap();
                let z1 = DiZhi::new(&z).unwrap();
                let gz1 = GanZhi::new(g1, z1).unwrap();
                assert_ne!(gz0, gz1, "{} != {}", gz0, gz1);
            }
        }
    }

    // 干支 + 整数
    //用数学归纳法
    #[test]
    fn test_gan_zhi_plus() {
        for (i, name) in GAN_ZHI_NAME.iter().enumerate() {
            let mut gz = GAN_ZHI_NAME[(i + 1) % 60].chars();

            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();

            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz0 = GanZhi::new(g0, z0).unwrap();

            let mut gz = name.chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();

            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz1 = GanZhi::new(g0, z0).unwrap();
            let gz1 = gz1.plus(1);
            assert_eq!(gz0, gz1, "{} + 1 = {}", gz1, gz0);
        }

        for (i, name) in GAN_ZHI_NAME.iter().enumerate() {
            let mut gz = GAN_ZHI_NAME[(60 + i - 1) % 60].chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();

            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz0 = GanZhi::new(g0, z0).unwrap();

            let mut gz = name.chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();
            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz1 = GanZhi::new(g0, z0).unwrap();
            let gz1 = gz1.plus(-1);

            assert_eq!(gz0, gz1, "{} + (-1) = {}", gz1, gz0);
        }
        for name in GAN_ZHI_NAME {
            let mut gz = name.chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();
            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz = GanZhi::new(g0, z0).unwrap();

            assert_eq!(
                gz.plus(99).plus(1),
                gz.plus(100),
                "{} + 99 + 1 = {} + 100",
                gz,
                gz
            );

            assert_eq!(
                gz.plus(-99).plus(-1),
                gz.plus(-100),
                "{} + (-99) + (-1) = {} + (-100)",
                gz,
                gz
            );
        }
    }
    // 干支 - 干支 = 正整数
    #[test]
    fn test_gan_zhi_minus() {
        for name in GAN_ZHI_NAME {
            let mut gz = name.chars();
            let g = gz.next().unwrap().to_string();
            let z = gz.next().unwrap().to_string();
            let g0 = TianGan::new(&g).unwrap();
            let z0 = DiZhi::new(&z).unwrap();
            let gz0 = GanZhi::new(g0, z0).unwrap();

            for (j, _) in GAN_ZHI_NAME.iter().enumerate() {
                let gz1 = gz0.plus(j as isize);
                assert_eq!(j, gz1.minus(&gz0) as usize, "{} - {} = {}", gz1, gz0, j);
            }
        }
    }
    #[test]
    fn test_default() {
        let default: GanZhi = Default::default();
        let g = TianGan::new("甲").unwrap();
        let z = DiZhi::new("子").unwrap();
        let gz = GanZhi::new(g, z).unwrap();
        assert_eq!(default, gz);
    }
}
