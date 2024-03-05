use std::fmt::Display;

use crate::{dizhi::DiZhi, tiangan::TianGan, wuxing::WuXing};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq, Copy)]
pub enum GanZhi {
    #[default]
    甲子,
    乙丑,
    丙寅,
    丁卯,
    戊辰,
    己巳,
    庚午,
    辛未,
    壬申,
    癸酉,
    甲戌,
    乙亥,
    丙子,
    丁丑,
    戊寅,
    己卯,
    庚辰,
    辛巳,
    壬午,
    癸未,
    甲申,
    乙酉,
    丙戌,
    丁亥,
    戊子,
    己丑,
    庚寅,
    辛卯,
    壬辰,
    癸巳,
    甲午,
    乙未,
    丙申,
    丁酉,
    戊戌,
    己亥,
    庚子,
    辛丑,
    壬寅,
    癸卯,
    甲辰,
    乙巳,
    丙午,
    丁未,
    戊申,
    己酉,
    庚戌,
    辛亥,
    壬子,
    癸丑,
    甲寅,
    乙卯,
    丙辰,
    丁巳,
    戊午,
    己未,
    庚申,
    辛酉,
    壬戌,
    癸亥,
}

impl GanZhi {
    pub fn new(g: &TianGan, z: &DiZhi) -> Result<GanZhi, String> {
        match (g, z) {
            (TianGan::甲, DiZhi::子) => Ok(GanZhi::甲子),
            (TianGan::乙, DiZhi::丑) => Ok(GanZhi::乙丑),
            (TianGan::丙, DiZhi::寅) => Ok(GanZhi::丙寅),
            (TianGan::丁, DiZhi::卯) => Ok(GanZhi::丁卯),
            (TianGan::戊, DiZhi::辰) => Ok(GanZhi::戊辰),
            (TianGan::己, DiZhi::巳) => Ok(GanZhi::己巳),
            (TianGan::庚, DiZhi::午) => Ok(GanZhi::庚午),
            (TianGan::辛, DiZhi::未) => Ok(GanZhi::辛未),
            (TianGan::壬, DiZhi::申) => Ok(GanZhi::壬申),
            (TianGan::癸, DiZhi::酉) => Ok(GanZhi::癸酉),
            (TianGan::甲, DiZhi::戌) => Ok(GanZhi::甲戌),
            (TianGan::乙, DiZhi::亥) => Ok(GanZhi::乙亥),
            (TianGan::丙, DiZhi::子) => Ok(GanZhi::丙子),
            (TianGan::丁, DiZhi::丑) => Ok(GanZhi::丁丑),
            (TianGan::戊, DiZhi::寅) => Ok(GanZhi::戊寅),
            (TianGan::己, DiZhi::卯) => Ok(GanZhi::己卯),
            (TianGan::庚, DiZhi::辰) => Ok(GanZhi::庚辰),
            (TianGan::辛, DiZhi::巳) => Ok(GanZhi::辛巳),
            (TianGan::壬, DiZhi::午) => Ok(GanZhi::壬午),
            (TianGan::癸, DiZhi::未) => Ok(GanZhi::癸未),
            (TianGan::甲, DiZhi::申) => Ok(GanZhi::甲申),
            (TianGan::乙, DiZhi::酉) => Ok(GanZhi::乙酉),
            (TianGan::丙, DiZhi::戌) => Ok(GanZhi::丙戌),
            (TianGan::丁, DiZhi::亥) => Ok(GanZhi::丁亥),
            (TianGan::戊, DiZhi::子) => Ok(GanZhi::戊子),
            (TianGan::己, DiZhi::丑) => Ok(GanZhi::己丑),
            (TianGan::庚, DiZhi::寅) => Ok(GanZhi::庚寅),
            (TianGan::辛, DiZhi::卯) => Ok(GanZhi::辛卯),
            (TianGan::壬, DiZhi::辰) => Ok(GanZhi::壬辰),
            (TianGan::癸, DiZhi::巳) => Ok(GanZhi::癸巳),
            (TianGan::甲, DiZhi::午) => Ok(GanZhi::甲午),
            (TianGan::乙, DiZhi::未) => Ok(GanZhi::乙未),
            (TianGan::丙, DiZhi::申) => Ok(GanZhi::丙申),
            (TianGan::丁, DiZhi::酉) => Ok(GanZhi::丁酉),
            (TianGan::戊, DiZhi::戌) => Ok(GanZhi::戊戌),
            (TianGan::己, DiZhi::亥) => Ok(GanZhi::己亥),
            (TianGan::庚, DiZhi::子) => Ok(GanZhi::庚子),
            (TianGan::辛, DiZhi::丑) => Ok(GanZhi::辛丑),
            (TianGan::壬, DiZhi::寅) => Ok(GanZhi::壬寅),
            (TianGan::癸, DiZhi::卯) => Ok(GanZhi::癸卯),
            (TianGan::甲, DiZhi::辰) => Ok(GanZhi::甲辰),
            (TianGan::乙, DiZhi::巳) => Ok(GanZhi::乙巳),
            (TianGan::丙, DiZhi::午) => Ok(GanZhi::丙午),
            (TianGan::丁, DiZhi::未) => Ok(GanZhi::丁未),
            (TianGan::戊, DiZhi::申) => Ok(GanZhi::戊申),
            (TianGan::己, DiZhi::酉) => Ok(GanZhi::己酉),
            (TianGan::庚, DiZhi::戌) => Ok(GanZhi::庚戌),
            (TianGan::辛, DiZhi::亥) => Ok(GanZhi::辛亥),
            (TianGan::壬, DiZhi::子) => Ok(GanZhi::壬子),
            (TianGan::癸, DiZhi::丑) => Ok(GanZhi::癸丑),
            (TianGan::甲, DiZhi::寅) => Ok(GanZhi::甲寅),
            (TianGan::乙, DiZhi::卯) => Ok(GanZhi::乙卯),
            (TianGan::丙, DiZhi::辰) => Ok(GanZhi::丙辰),
            (TianGan::丁, DiZhi::巳) => Ok(GanZhi::丁巳),
            (TianGan::戊, DiZhi::午) => Ok(GanZhi::戊午),
            (TianGan::己, DiZhi::未) => Ok(GanZhi::己未),
            (TianGan::庚, DiZhi::申) => Ok(GanZhi::庚申),
            (TianGan::辛, DiZhi::酉) => Ok(GanZhi::辛酉),
            (TianGan::壬, DiZhi::戌) => Ok(GanZhi::壬戌),
            (TianGan::癸, DiZhi::亥) => Ok(GanZhi::癸亥),
            _ => Err(format!("{}{}不能组成干支！", g, z)),
        }
    }
    pub fn gan(&self) -> TianGan {
        match self {
            GanZhi::甲子
            | GanZhi::甲戌
            | GanZhi::甲申
            | GanZhi::甲午
            | GanZhi::甲辰
            | GanZhi::甲寅 => TianGan::甲,

            GanZhi::乙丑
            | GanZhi::乙亥
            | GanZhi::乙酉
            | GanZhi::乙未
            | GanZhi::乙巳
            | GanZhi::乙卯 => TianGan::乙,

            GanZhi::丙寅
            | GanZhi::丙子
            | GanZhi::丙戌
            | GanZhi::丙申
            | GanZhi::丙午
            | GanZhi::丙辰 => TianGan::丙,

            GanZhi::丁卯
            | GanZhi::丁丑
            | GanZhi::丁亥
            | GanZhi::丁酉
            | GanZhi::丁未
            | GanZhi::丁巳 => TianGan::丁,

            GanZhi::戊辰
            | GanZhi::戊寅
            | GanZhi::戊子
            | GanZhi::戊戌
            | GanZhi::戊申
            | GanZhi::戊午 => TianGan::戊,

            GanZhi::己巳
            | GanZhi::己卯
            | GanZhi::己丑
            | GanZhi::己亥
            | GanZhi::己酉
            | GanZhi::己未 => TianGan::己,

            GanZhi::庚午
            | GanZhi::庚辰
            | GanZhi::庚寅
            | GanZhi::庚子
            | GanZhi::庚戌
            | GanZhi::庚申 => TianGan::庚,

            GanZhi::辛未
            | GanZhi::辛巳
            | GanZhi::辛卯
            | GanZhi::辛丑
            | GanZhi::辛亥
            | GanZhi::辛酉 => TianGan::辛,

            GanZhi::壬申
            | GanZhi::壬午
            | GanZhi::壬辰
            | GanZhi::壬寅
            | GanZhi::壬子
            | GanZhi::壬戌 => TianGan::壬,

            GanZhi::癸酉
            | GanZhi::癸未
            | GanZhi::癸巳
            | GanZhi::癸卯
            | GanZhi::癸丑
            | GanZhi::癸亥 => TianGan::癸,
        }
    }

    pub fn zhi(&self) -> DiZhi {
        match self {
            GanZhi::甲子 | GanZhi::丙子 | GanZhi::戊子 | GanZhi::庚子 | GanZhi::壬子 => {
                DiZhi::子
            }

            GanZhi::乙丑 | GanZhi::丁丑 | GanZhi::己丑 | GanZhi::辛丑 | GanZhi::癸丑 => {
                DiZhi::丑
            }

            GanZhi::丙寅 | GanZhi::戊寅 | GanZhi::庚寅 | GanZhi::壬寅 | GanZhi::甲寅 => {
                DiZhi::寅
            }

            GanZhi::丁卯 | GanZhi::己卯 | GanZhi::辛卯 | GanZhi::癸卯 | GanZhi::乙卯 => {
                DiZhi::卯
            }

            GanZhi::戊辰 | GanZhi::庚辰 | GanZhi::壬辰 | GanZhi::甲辰 | GanZhi::丙辰 => {
                DiZhi::辰
            }

            GanZhi::己巳 | GanZhi::辛巳 | GanZhi::癸巳 | GanZhi::乙巳 | GanZhi::丁巳 => {
                DiZhi::巳
            }

            GanZhi::庚午 | GanZhi::壬午 | GanZhi::甲午 | GanZhi::丙午 | GanZhi::戊午 => {
                DiZhi::午
            }

            GanZhi::辛未 | GanZhi::癸未 | GanZhi::乙未 | GanZhi::丁未 | GanZhi::己未 => {
                DiZhi::未
            }

            GanZhi::壬申 | GanZhi::甲申 | GanZhi::丙申 | GanZhi::戊申 | GanZhi::庚申 => {
                DiZhi::申
            }

            GanZhi::癸酉 | GanZhi::乙酉 | GanZhi::丁酉 | GanZhi::己酉 | GanZhi::辛酉 => {
                DiZhi::酉
            }

            GanZhi::甲戌 | GanZhi::丙戌 | GanZhi::戊戌 | GanZhi::庚戌 | GanZhi::壬戌 => {
                DiZhi::戌
            }

            GanZhi::乙亥 | GanZhi::丁亥 | GanZhi::己亥 | GanZhi::辛亥 | GanZhi::癸亥 => {
                DiZhi::亥
            }
        }
    }

    /// 纳音
    pub fn na_yin(&self) -> WuXing {
        let gan = self.gan();
        let zhi = self.zhi();
        let g = if gan.masculine() {
            gan.plus(1)
        } else {
            gan.plus(-1)
        };
        let z = if zhi.masculine() {
            zhi.plus(1)
        } else {
            zhi.plus(-1)
        };
        let n = (49 - (gan.tai_xuan() + zhi.tai_xuan() + g.tai_xuan() + z.tai_xuan())) % 10;
        let w = if n == 1 || n == 6 {
            WuXing::水
        } else if n == 2 || n == 7 {
            WuXing::火
        } else if n == 3 || n == 8 {
            WuXing::木
        } else if n == 4 || n == 9 {
            WuXing::金
        } else {
            WuXing::土
        };

        // w所生者即为所求纳音
        // 依序为w的下一个
        match w {
            WuXing::木 => WuXing::火,
            WuXing::火 => WuXing::土,
            WuXing::土 => WuXing::金,
            WuXing::金 => WuXing::水,
            WuXing::水 => WuXing::木,
        }
    }

    pub fn plus(&self, other: isize) -> Self {
        let g = self.gan().plus(other);
        let z = self.zhi().plus(other);

        match (g, z) {
            (TianGan::甲, DiZhi::子) => GanZhi::甲子,
            (TianGan::乙, DiZhi::丑) => GanZhi::乙丑,
            (TianGan::丙, DiZhi::寅) => GanZhi::丙寅,
            (TianGan::丁, DiZhi::卯) => GanZhi::丁卯,
            (TianGan::戊, DiZhi::辰) => GanZhi::戊辰,
            (TianGan::己, DiZhi::巳) => GanZhi::己巳,
            (TianGan::庚, DiZhi::午) => GanZhi::庚午,
            (TianGan::辛, DiZhi::未) => GanZhi::辛未,
            (TianGan::壬, DiZhi::申) => GanZhi::壬申,
            (TianGan::癸, DiZhi::酉) => GanZhi::癸酉,
            (TianGan::甲, DiZhi::戌) => GanZhi::甲戌,
            (TianGan::乙, DiZhi::亥) => GanZhi::乙亥,
            (TianGan::丙, DiZhi::子) => GanZhi::丙子,
            (TianGan::丁, DiZhi::丑) => GanZhi::丁丑,
            (TianGan::戊, DiZhi::寅) => GanZhi::戊寅,
            (TianGan::己, DiZhi::卯) => GanZhi::己卯,
            (TianGan::庚, DiZhi::辰) => GanZhi::庚辰,
            (TianGan::辛, DiZhi::巳) => GanZhi::辛巳,
            (TianGan::壬, DiZhi::午) => GanZhi::壬午,
            (TianGan::癸, DiZhi::未) => GanZhi::癸未,
            (TianGan::甲, DiZhi::申) => GanZhi::甲申,
            (TianGan::乙, DiZhi::酉) => GanZhi::乙酉,
            (TianGan::丙, DiZhi::戌) => GanZhi::丙戌,
            (TianGan::丁, DiZhi::亥) => GanZhi::丁亥,
            (TianGan::戊, DiZhi::子) => GanZhi::戊子,
            (TianGan::己, DiZhi::丑) => GanZhi::己丑,
            (TianGan::庚, DiZhi::寅) => GanZhi::庚寅,
            (TianGan::辛, DiZhi::卯) => GanZhi::辛卯,
            (TianGan::壬, DiZhi::辰) => GanZhi::壬辰,
            (TianGan::癸, DiZhi::巳) => GanZhi::癸巳,
            (TianGan::甲, DiZhi::午) => GanZhi::甲午,
            (TianGan::乙, DiZhi::未) => GanZhi::乙未,
            (TianGan::丙, DiZhi::申) => GanZhi::丙申,
            (TianGan::丁, DiZhi::酉) => GanZhi::丁酉,
            (TianGan::戊, DiZhi::戌) => GanZhi::戊戌,
            (TianGan::己, DiZhi::亥) => GanZhi::己亥,
            (TianGan::庚, DiZhi::子) => GanZhi::庚子,
            (TianGan::辛, DiZhi::丑) => GanZhi::辛丑,
            (TianGan::壬, DiZhi::寅) => GanZhi::壬寅,
            (TianGan::癸, DiZhi::卯) => GanZhi::癸卯,
            (TianGan::甲, DiZhi::辰) => GanZhi::甲辰,
            (TianGan::乙, DiZhi::巳) => GanZhi::乙巳,
            (TianGan::丙, DiZhi::午) => GanZhi::丙午,
            (TianGan::丁, DiZhi::未) => GanZhi::丁未,
            (TianGan::戊, DiZhi::申) => GanZhi::戊申,
            (TianGan::己, DiZhi::酉) => GanZhi::己酉,
            (TianGan::庚, DiZhi::戌) => GanZhi::庚戌,
            (TianGan::辛, DiZhi::亥) => GanZhi::辛亥,
            (TianGan::壬, DiZhi::子) => GanZhi::壬子,
            (TianGan::癸, DiZhi::丑) => GanZhi::癸丑,
            (TianGan::甲, DiZhi::寅) => GanZhi::甲寅,
            (TianGan::乙, DiZhi::卯) => GanZhi::乙卯,
            (TianGan::丙, DiZhi::辰) => GanZhi::丙辰,
            (TianGan::丁, DiZhi::巳) => GanZhi::丁巳,
            (TianGan::戊, DiZhi::午) => GanZhi::戊午,
            (TianGan::己, DiZhi::未) => GanZhi::己未,
            (TianGan::庚, DiZhi::申) => GanZhi::庚申,
            (TianGan::辛, DiZhi::酉) => GanZhi::辛酉,
            (TianGan::壬, DiZhi::戌) => GanZhi::壬戌,
            _ => GanZhi::癸亥,
        }
    }

    pub fn minus(&self, other: &GanZhi) -> u8 {
        // 返回整数值
        (60 + self.clone() as u8 - other.clone() as u8) % 60
    }
}

impl Display for GanZhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.gan(), self.zhi())
    }
}

#[cfg(test)]
mod tests {

    use crate::{dizhi::DiZhi, tiangan::TianGan, wuxing::WuXing};

    use super::GanZhi;

    const GAN_ZHI_NAME: [GanZhi; 60] = [
        GanZhi::甲子,
        GanZhi::乙丑,
        GanZhi::丙寅,
        GanZhi::丁卯,
        GanZhi::戊辰,
        GanZhi::己巳,
        GanZhi::庚午,
        GanZhi::辛未,
        GanZhi::壬申,
        GanZhi::癸酉,
        GanZhi::甲戌,
        GanZhi::乙亥,
        GanZhi::丙子,
        GanZhi::丁丑,
        GanZhi::戊寅,
        GanZhi::己卯,
        GanZhi::庚辰,
        GanZhi::辛巳,
        GanZhi::壬午,
        GanZhi::癸未,
        GanZhi::甲申,
        GanZhi::乙酉,
        GanZhi::丙戌,
        GanZhi::丁亥,
        GanZhi::戊子,
        GanZhi::己丑,
        GanZhi::庚寅,
        GanZhi::辛卯,
        GanZhi::壬辰,
        GanZhi::癸巳,
        GanZhi::甲午,
        GanZhi::乙未,
        GanZhi::丙申,
        GanZhi::丁酉,
        GanZhi::戊戌,
        GanZhi::己亥,
        GanZhi::庚子,
        GanZhi::辛丑,
        GanZhi::壬寅,
        GanZhi::癸卯,
        GanZhi::甲辰,
        GanZhi::乙巳,
        GanZhi::丙午,
        GanZhi::丁未,
        GanZhi::戊申,
        GanZhi::己酉,
        GanZhi::庚戌,
        GanZhi::辛亥,
        GanZhi::壬子,
        GanZhi::癸丑,
        GanZhi::甲寅,
        GanZhi::乙卯,
        GanZhi::丙辰,
        GanZhi::丁巳,
        GanZhi::戊午,
        GanZhi::己未,
        GanZhi::庚申,
        GanZhi::辛酉,
        GanZhi::壬戌,
        GanZhi::癸亥,
    ];

    #[test]
    fn test_new() {
        let gan = [
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

        let zhi = [
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

        for g in gan {
            for z in &zhi {
                let gz = GanZhi::new(&g, z);
                if g.masculine() == z.masculine() {
                    assert!(gz.is_ok());
                    let gz = gz.unwrap();
                    assert_eq!(g, gz.gan());
                    assert_eq!(*z, gz.zhi());
                } else {
                    assert!(gz.is_err());
                }
            }
        }
    }

    #[test]
    fn test_gan() {
        assert_eq!(GanZhi::甲子.gan(), TianGan::甲);
        assert_eq!(GanZhi::乙丑.gan(), TianGan::乙);
        assert_eq!(GanZhi::丙寅.gan(), TianGan::丙);
        assert_eq!(GanZhi::丁卯.gan(), TianGan::丁);
        assert_eq!(GanZhi::戊辰.gan(), TianGan::戊);
        assert_eq!(GanZhi::己巳.gan(), TianGan::己);
        assert_eq!(GanZhi::庚午.gan(), TianGan::庚);
        assert_eq!(GanZhi::辛未.gan(), TianGan::辛);
        assert_eq!(GanZhi::壬申.gan(), TianGan::壬);
        assert_eq!(GanZhi::癸酉.gan(), TianGan::癸);
        assert_eq!(GanZhi::甲戌.gan(), TianGan::甲);
        assert_eq!(GanZhi::乙亥.gan(), TianGan::乙);
        assert_eq!(GanZhi::丙子.gan(), TianGan::丙);
        assert_eq!(GanZhi::丁丑.gan(), TianGan::丁);
        assert_eq!(GanZhi::戊寅.gan(), TianGan::戊);
        assert_eq!(GanZhi::己卯.gan(), TianGan::己);
        assert_eq!(GanZhi::庚辰.gan(), TianGan::庚);
        assert_eq!(GanZhi::辛巳.gan(), TianGan::辛);
        assert_eq!(GanZhi::壬午.gan(), TianGan::壬);
        assert_eq!(GanZhi::癸未.gan(), TianGan::癸);
        assert_eq!(GanZhi::甲申.gan(), TianGan::甲);
        assert_eq!(GanZhi::乙酉.gan(), TianGan::乙);
        assert_eq!(GanZhi::丙戌.gan(), TianGan::丙);
        assert_eq!(GanZhi::丁亥.gan(), TianGan::丁);
        assert_eq!(GanZhi::戊子.gan(), TianGan::戊);
        assert_eq!(GanZhi::己丑.gan(), TianGan::己);
        assert_eq!(GanZhi::庚寅.gan(), TianGan::庚);
        assert_eq!(GanZhi::辛卯.gan(), TianGan::辛);
        assert_eq!(GanZhi::壬辰.gan(), TianGan::壬);
        assert_eq!(GanZhi::癸巳.gan(), TianGan::癸);
        assert_eq!(GanZhi::甲午.gan(), TianGan::甲);
        assert_eq!(GanZhi::乙未.gan(), TianGan::乙);
        assert_eq!(GanZhi::丙申.gan(), TianGan::丙);
        assert_eq!(GanZhi::丁酉.gan(), TianGan::丁);
        assert_eq!(GanZhi::戊戌.gan(), TianGan::戊);
        assert_eq!(GanZhi::己亥.gan(), TianGan::己);
        assert_eq!(GanZhi::庚子.gan(), TianGan::庚);
        assert_eq!(GanZhi::辛丑.gan(), TianGan::辛);
        assert_eq!(GanZhi::壬寅.gan(), TianGan::壬);
        assert_eq!(GanZhi::癸卯.gan(), TianGan::癸);
        assert_eq!(GanZhi::甲辰.gan(), TianGan::甲);
        assert_eq!(GanZhi::乙巳.gan(), TianGan::乙);
        assert_eq!(GanZhi::丙午.gan(), TianGan::丙);
        assert_eq!(GanZhi::丁未.gan(), TianGan::丁);
        assert_eq!(GanZhi::戊申.gan(), TianGan::戊);
        assert_eq!(GanZhi::己酉.gan(), TianGan::己);
        assert_eq!(GanZhi::庚戌.gan(), TianGan::庚);
        assert_eq!(GanZhi::辛亥.gan(), TianGan::辛);
        assert_eq!(GanZhi::壬子.gan(), TianGan::壬);
        assert_eq!(GanZhi::癸丑.gan(), TianGan::癸);
        assert_eq!(GanZhi::甲寅.gan(), TianGan::甲);
        assert_eq!(GanZhi::乙卯.gan(), TianGan::乙);
        assert_eq!(GanZhi::丙辰.gan(), TianGan::丙);
        assert_eq!(GanZhi::丁巳.gan(), TianGan::丁);
        assert_eq!(GanZhi::戊午.gan(), TianGan::戊);
        assert_eq!(GanZhi::己未.gan(), TianGan::己);
        assert_eq!(GanZhi::庚申.gan(), TianGan::庚);
        assert_eq!(GanZhi::辛酉.gan(), TianGan::辛);
        assert_eq!(GanZhi::壬戌.gan(), TianGan::壬);
        assert_eq!(GanZhi::癸亥.gan(), TianGan::癸);
    }

    #[test]
    fn test_zhi() {
        assert_eq!(GanZhi::甲子.zhi(), DiZhi::子);
        assert_eq!(GanZhi::乙丑.zhi(), DiZhi::丑);
        assert_eq!(GanZhi::丙寅.zhi(), DiZhi::寅);
        assert_eq!(GanZhi::丁卯.zhi(), DiZhi::卯);
        assert_eq!(GanZhi::戊辰.zhi(), DiZhi::辰);
        assert_eq!(GanZhi::己巳.zhi(), DiZhi::巳);
        assert_eq!(GanZhi::庚午.zhi(), DiZhi::午);
        assert_eq!(GanZhi::辛未.zhi(), DiZhi::未);
        assert_eq!(GanZhi::壬申.zhi(), DiZhi::申);
        assert_eq!(GanZhi::癸酉.zhi(), DiZhi::酉);
        assert_eq!(GanZhi::甲戌.zhi(), DiZhi::戌);
        assert_eq!(GanZhi::乙亥.zhi(), DiZhi::亥);
        assert_eq!(GanZhi::丙子.zhi(), DiZhi::子);
        assert_eq!(GanZhi::丁丑.zhi(), DiZhi::丑);
        assert_eq!(GanZhi::戊寅.zhi(), DiZhi::寅);
        assert_eq!(GanZhi::己卯.zhi(), DiZhi::卯);
        assert_eq!(GanZhi::庚辰.zhi(), DiZhi::辰);
        assert_eq!(GanZhi::辛巳.zhi(), DiZhi::巳);
        assert_eq!(GanZhi::壬午.zhi(), DiZhi::午);
        assert_eq!(GanZhi::癸未.zhi(), DiZhi::未);
        assert_eq!(GanZhi::甲申.zhi(), DiZhi::申);
        assert_eq!(GanZhi::乙酉.zhi(), DiZhi::酉);
        assert_eq!(GanZhi::丙戌.zhi(), DiZhi::戌);
        assert_eq!(GanZhi::丁亥.zhi(), DiZhi::亥);
        assert_eq!(GanZhi::戊子.zhi(), DiZhi::子);
        assert_eq!(GanZhi::己丑.zhi(), DiZhi::丑);
        assert_eq!(GanZhi::庚寅.zhi(), DiZhi::寅);
        assert_eq!(GanZhi::辛卯.zhi(), DiZhi::卯);
        assert_eq!(GanZhi::壬辰.zhi(), DiZhi::辰);
        assert_eq!(GanZhi::癸巳.zhi(), DiZhi::巳);
        assert_eq!(GanZhi::甲午.zhi(), DiZhi::午);
        assert_eq!(GanZhi::乙未.zhi(), DiZhi::未);
        assert_eq!(GanZhi::丙申.zhi(), DiZhi::申);
        assert_eq!(GanZhi::丁酉.zhi(), DiZhi::酉);
        assert_eq!(GanZhi::戊戌.zhi(), DiZhi::戌);
        assert_eq!(GanZhi::己亥.zhi(), DiZhi::亥);
        assert_eq!(GanZhi::庚子.zhi(), DiZhi::子);
        assert_eq!(GanZhi::辛丑.zhi(), DiZhi::丑);
        assert_eq!(GanZhi::壬寅.zhi(), DiZhi::寅);
        assert_eq!(GanZhi::癸卯.zhi(), DiZhi::卯);
        assert_eq!(GanZhi::甲辰.zhi(), DiZhi::辰);
        assert_eq!(GanZhi::乙巳.zhi(), DiZhi::巳);
        assert_eq!(GanZhi::丙午.zhi(), DiZhi::午);
        assert_eq!(GanZhi::丁未.zhi(), DiZhi::未);
        assert_eq!(GanZhi::戊申.zhi(), DiZhi::申);
        assert_eq!(GanZhi::己酉.zhi(), DiZhi::酉);
        assert_eq!(GanZhi::庚戌.zhi(), DiZhi::戌);
        assert_eq!(GanZhi::辛亥.zhi(), DiZhi::亥);
        assert_eq!(GanZhi::壬子.zhi(), DiZhi::子);
        assert_eq!(GanZhi::癸丑.zhi(), DiZhi::丑);
        assert_eq!(GanZhi::甲寅.zhi(), DiZhi::寅);
        assert_eq!(GanZhi::乙卯.zhi(), DiZhi::卯);
        assert_eq!(GanZhi::丙辰.zhi(), DiZhi::辰);
        assert_eq!(GanZhi::丁巳.zhi(), DiZhi::巳);
        assert_eq!(GanZhi::戊午.zhi(), DiZhi::午);
        assert_eq!(GanZhi::己未.zhi(), DiZhi::未);
        assert_eq!(GanZhi::庚申.zhi(), DiZhi::申);
        assert_eq!(GanZhi::辛酉.zhi(), DiZhi::酉);
        assert_eq!(GanZhi::壬戌.zhi(), DiZhi::戌);
        assert_eq!(GanZhi::癸亥.zhi(), DiZhi::亥);
    }

    #[test]
    fn test_na_yin() {
        assert_eq!(GanZhi::甲子.na_yin(), WuXing::金);
        assert_eq!(GanZhi::乙丑.na_yin(), WuXing::金);
        assert_eq!(GanZhi::丙寅.na_yin(), WuXing::火);
        assert_eq!(GanZhi::丁卯.na_yin(), WuXing::火);
        assert_eq!(GanZhi::戊辰.na_yin(), WuXing::木);
        assert_eq!(GanZhi::己巳.na_yin(), WuXing::木);
        assert_eq!(GanZhi::庚午.na_yin(), WuXing::土);
        assert_eq!(GanZhi::辛未.na_yin(), WuXing::土);
        assert_eq!(GanZhi::壬申.na_yin(), WuXing::金);
        assert_eq!(GanZhi::癸酉.na_yin(), WuXing::金);
        assert_eq!(GanZhi::甲戌.na_yin(), WuXing::火);
        assert_eq!(GanZhi::乙亥.na_yin(), WuXing::火);
        assert_eq!(GanZhi::丙子.na_yin(), WuXing::水);
        assert_eq!(GanZhi::丁丑.na_yin(), WuXing::水);
        assert_eq!(GanZhi::戊寅.na_yin(), WuXing::土);
        assert_eq!(GanZhi::己卯.na_yin(), WuXing::土);
        assert_eq!(GanZhi::庚辰.na_yin(), WuXing::金);
        assert_eq!(GanZhi::辛巳.na_yin(), WuXing::金);
        assert_eq!(GanZhi::壬午.na_yin(), WuXing::木);
        assert_eq!(GanZhi::癸未.na_yin(), WuXing::木);
        assert_eq!(GanZhi::甲申.na_yin(), WuXing::水);
        assert_eq!(GanZhi::乙酉.na_yin(), WuXing::水);
        assert_eq!(GanZhi::丙戌.na_yin(), WuXing::土);
        assert_eq!(GanZhi::丁亥.na_yin(), WuXing::土);
        assert_eq!(GanZhi::戊子.na_yin(), WuXing::火);
        assert_eq!(GanZhi::己丑.na_yin(), WuXing::火);
        assert_eq!(GanZhi::庚寅.na_yin(), WuXing::木);
        assert_eq!(GanZhi::辛卯.na_yin(), WuXing::木);
        assert_eq!(GanZhi::壬辰.na_yin(), WuXing::水);
        assert_eq!(GanZhi::癸巳.na_yin(), WuXing::水);
        assert_eq!(GanZhi::甲午.na_yin(), WuXing::金);
        assert_eq!(GanZhi::乙未.na_yin(), WuXing::金);
        assert_eq!(GanZhi::丙申.na_yin(), WuXing::火);
        assert_eq!(GanZhi::丁酉.na_yin(), WuXing::火);
        assert_eq!(GanZhi::戊戌.na_yin(), WuXing::木);
        assert_eq!(GanZhi::己亥.na_yin(), WuXing::木);
        assert_eq!(GanZhi::庚子.na_yin(), WuXing::土);
        assert_eq!(GanZhi::辛丑.na_yin(), WuXing::土);
        assert_eq!(GanZhi::壬寅.na_yin(), WuXing::金);
        assert_eq!(GanZhi::癸卯.na_yin(), WuXing::金);
        assert_eq!(GanZhi::甲辰.na_yin(), WuXing::火);
        assert_eq!(GanZhi::乙巳.na_yin(), WuXing::火);
        assert_eq!(GanZhi::丙午.na_yin(), WuXing::水);
        assert_eq!(GanZhi::丁未.na_yin(), WuXing::水);
        assert_eq!(GanZhi::戊申.na_yin(), WuXing::土);
        assert_eq!(GanZhi::己酉.na_yin(), WuXing::土);
        assert_eq!(GanZhi::庚戌.na_yin(), WuXing::金);
        assert_eq!(GanZhi::辛亥.na_yin(), WuXing::金);
        assert_eq!(GanZhi::壬子.na_yin(), WuXing::木);
        assert_eq!(GanZhi::癸丑.na_yin(), WuXing::木);
        assert_eq!(GanZhi::甲寅.na_yin(), WuXing::水);
        assert_eq!(GanZhi::乙卯.na_yin(), WuXing::水);
        assert_eq!(GanZhi::丙辰.na_yin(), WuXing::土);
        assert_eq!(GanZhi::丁巳.na_yin(), WuXing::土);
        assert_eq!(GanZhi::戊午.na_yin(), WuXing::火);
        assert_eq!(GanZhi::己未.na_yin(), WuXing::火);
        assert_eq!(GanZhi::庚申.na_yin(), WuXing::木);
        assert_eq!(GanZhi::辛酉.na_yin(), WuXing::木);
        assert_eq!(GanZhi::壬戌.na_yin(), WuXing::水);
        assert_eq!(GanZhi::癸亥.na_yin(), WuXing::水);
    }

    // 干支相等
    #[test]
    fn test_gan_zhi_equals() {
        assert_eq!(GanZhi::甲子, GanZhi::甲子);
        assert_eq!(GanZhi::乙丑, GanZhi::乙丑);
        assert_eq!(GanZhi::丙寅, GanZhi::丙寅);
        assert_eq!(GanZhi::丁卯, GanZhi::丁卯);
        assert_eq!(GanZhi::戊辰, GanZhi::戊辰);
        assert_eq!(GanZhi::己巳, GanZhi::己巳);
        assert_eq!(GanZhi::庚午, GanZhi::庚午);
        assert_eq!(GanZhi::辛未, GanZhi::辛未);
        assert_eq!(GanZhi::壬申, GanZhi::壬申);
        assert_eq!(GanZhi::癸酉, GanZhi::癸酉);
        assert_eq!(GanZhi::甲戌, GanZhi::甲戌);
        assert_eq!(GanZhi::乙亥, GanZhi::乙亥);
        assert_eq!(GanZhi::丙子, GanZhi::丙子);
        assert_eq!(GanZhi::丁丑, GanZhi::丁丑);
        assert_eq!(GanZhi::戊寅, GanZhi::戊寅);
        assert_eq!(GanZhi::己卯, GanZhi::己卯);
        assert_eq!(GanZhi::庚辰, GanZhi::庚辰);
        assert_eq!(GanZhi::辛巳, GanZhi::辛巳);
        assert_eq!(GanZhi::壬午, GanZhi::壬午);
        assert_eq!(GanZhi::癸未, GanZhi::癸未);
        assert_eq!(GanZhi::甲申, GanZhi::甲申);
        assert_eq!(GanZhi::乙酉, GanZhi::乙酉);
        assert_eq!(GanZhi::丙戌, GanZhi::丙戌);
        assert_eq!(GanZhi::丁亥, GanZhi::丁亥);
        assert_eq!(GanZhi::戊子, GanZhi::戊子);
        assert_eq!(GanZhi::己丑, GanZhi::己丑);
        assert_eq!(GanZhi::庚寅, GanZhi::庚寅);
        assert_eq!(GanZhi::辛卯, GanZhi::辛卯);
        assert_eq!(GanZhi::壬辰, GanZhi::壬辰);
        assert_eq!(GanZhi::癸巳, GanZhi::癸巳);
        assert_eq!(GanZhi::甲午, GanZhi::甲午);
        assert_eq!(GanZhi::乙未, GanZhi::乙未);
        assert_eq!(GanZhi::丙申, GanZhi::丙申);
        assert_eq!(GanZhi::丁酉, GanZhi::丁酉);
        assert_eq!(GanZhi::戊戌, GanZhi::戊戌);
        assert_eq!(GanZhi::己亥, GanZhi::己亥);
        assert_eq!(GanZhi::庚子, GanZhi::庚子);
        assert_eq!(GanZhi::辛丑, GanZhi::辛丑);
        assert_eq!(GanZhi::壬寅, GanZhi::壬寅);
        assert_eq!(GanZhi::癸卯, GanZhi::癸卯);
        assert_eq!(GanZhi::甲辰, GanZhi::甲辰);
        assert_eq!(GanZhi::乙巳, GanZhi::乙巳);
        assert_eq!(GanZhi::丙午, GanZhi::丙午);
        assert_eq!(GanZhi::丁未, GanZhi::丁未);
        assert_eq!(GanZhi::戊申, GanZhi::戊申);
        assert_eq!(GanZhi::己酉, GanZhi::己酉);
        assert_eq!(GanZhi::庚戌, GanZhi::庚戌);
        assert_eq!(GanZhi::辛亥, GanZhi::辛亥);
        assert_eq!(GanZhi::壬子, GanZhi::壬子);
        assert_eq!(GanZhi::癸丑, GanZhi::癸丑);
        assert_eq!(GanZhi::甲寅, GanZhi::甲寅);
        assert_eq!(GanZhi::乙卯, GanZhi::乙卯);
        assert_eq!(GanZhi::丙辰, GanZhi::丙辰);
        assert_eq!(GanZhi::丁巳, GanZhi::丁巳);
        assert_eq!(GanZhi::戊午, GanZhi::戊午);
        assert_eq!(GanZhi::己未, GanZhi::己未);
        assert_eq!(GanZhi::庚申, GanZhi::庚申);
        assert_eq!(GanZhi::辛酉, GanZhi::辛酉);
        assert_eq!(GanZhi::壬戌, GanZhi::壬戌);
        assert_eq!(GanZhi::癸亥, GanZhi::癸亥);
    }

    // 干支不相等
    #[test]
    fn test_gan_zhi_not_equals() {
        for (i, gz) in GAN_ZHI_NAME.iter().enumerate() {
            let gz0 = gz.clone();
            for j in 1..60 {
                let n = (i + j) % 60;
                let gz1 = GAN_ZHI_NAME[n].clone();
                assert_ne!(gz0, gz1);
            }
        }
    }

    // 干支 + 整数
    //用数学归纳法
    #[test]
    fn test_gan_zhi_plus() {
        for (i, gz) in GAN_ZHI_NAME.iter().enumerate() {
            let gz0 = GAN_ZHI_NAME[(i + 1) % 60].clone();
            let gz1 = gz.plus(1);

            assert_eq!(gz0, gz1, "{} + 1={}", gz, gz0);
        }

        for (i, gz) in GAN_ZHI_NAME.iter().enumerate() {
            let gz0 = GAN_ZHI_NAME[(60 + i - 1) % 60].clone();
            let gz1 = gz.plus(-1);

            assert_eq!(gz0, gz1, "{} - 1={}", gz, gz0);
        }
        for gz in GAN_ZHI_NAME {
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
        for gz0 in GAN_ZHI_NAME {
            for (j, _) in GAN_ZHI_NAME.iter().enumerate() {
                let gz1 = gz0.plus(j as isize);
                assert_eq!(j, gz1.minus(&gz0) as usize, "{} - {} = {}", gz1, gz0, j);
            }
        }
    }

    #[test]
    fn test_default() {
        let default: GanZhi = Default::default();
        assert_eq!(default, GanZhi::甲子);
    }
}
