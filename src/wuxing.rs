use std::fmt::Display;

pub(crate) const WU_XING_NUM_TO_NAME: [&str; 5] = ["木", "火", "土", "金", "水"];

#[derive(Debug, Eq, Clone)]
pub struct WuXing {
    pub name: String,
    pub(crate) num: u8,
}

impl WuXing {
    pub fn new(name: &str) -> Result<WuXing, String> {
        if let Some(num) = WU_XING_NUM_TO_NAME.iter().position(|&s| s == name) {
            Ok(WuXing {
                name: name.to_string(),
                num: num as u8 + 1,
            })
        } else {
            Err(format!("没有此五行：{}", name))
        }
    }

    /// 生
    pub fn sheng(&self, w2: &WuXing) -> bool {
        return (self.num as i16 - w2.num as i16 - 4) % 5 == 0;
    }

    /// 克
    pub fn ke(&self, w2: &WuXing) -> bool {
        return (self.num as i16 - w2.num as i16 - 3) % 5 == 0;
    }
}

impl Display for WuXing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for WuXing {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.num == other.num
    }
}

#[cfg(test)]
mod tests {

    use super::{WuXing, WU_XING_NUM_TO_NAME};
    const WU_XING_NAME: [&str; 5] = ["木", "火", "土", "金", "水"];
    #[test]

    fn test_new() {
        for (index, &value) in WU_XING_NUM_TO_NAME.iter().enumerate() {
            let wu_xing = WuXing::new(value);

            assert!(wu_xing.is_ok());
            let wu_xing = wu_xing.unwrap();
            assert_eq!(wu_xing.name, value);
            assert_eq!(wu_xing.num, index as u8 + 1);
        }

        let wu_xing = WuXing::new("否");
        assert!(wu_xing.is_err());
    }
    #[test]
    fn test_display() {
        for value in WU_XING_NUM_TO_NAME {
            let w = WuXing::new(value).unwrap();
            let w = format!("{}", w);
            assert_eq!(value, w);
        }
    }
    #[test]
    fn test_wu_xing_equals() {
        for name in WU_XING_NAME {
            let w0 = WuXing::new(name).unwrap();
            let w1 = WuXing::new(name).unwrap();
            assert_eq!(w0, w1);
            assert_eq!(w1, w0);
        }
    }

    #[test]
    fn test_wu_xing_not_equals() {
        for i in 0..WU_XING_NAME.len() {
            let w0 = WuXing::new(WU_XING_NAME[i]).unwrap();

            for j in 1..WU_XING_NAME.len() - 1 {
                let n = (i + j) % WU_XING_NAME.len();
                let w1 = WuXing::new(WU_XING_NAME[n]).unwrap();

                assert_ne!(w0, w1);
                assert_ne!(w1, w0);
            }
        }
    }

    // 五行相生
    #[test]
    fn test_wu_xing_sheng() {
        for (i, _) in WU_XING_NAME.iter().enumerate() {
            let n = (i + 1) % WU_XING_NAME.len();
            let w0 = WuXing::new(WU_XING_NAME[i]).unwrap();
            let w1 = WuXing::new(WU_XING_NAME[n]).unwrap();

            assert!(w0.sheng(&w1));
        }
    }

    // 五行相克
    #[test]
    fn test_wu_xing_ke() {
        for (i, _) in WU_XING_NAME.iter().enumerate() {
            let n = (i + 2) % WU_XING_NAME.len();
            let w0 = WuXing::new(WU_XING_NAME[i]).unwrap();
            let w1 = WuXing::new(WU_XING_NAME[n]).unwrap();

            assert!(w0.ke(&w1));
        }
    }

    #[test]
    fn test_wu_xing_not_sheng() {
        for (i, _) in WU_XING_NAME.iter().enumerate() {
            let n0 = (i + 2) % WU_XING_NAME.len();
            let n1 = (i + 3) % WU_XING_NAME.len();
            let n2 = (i + 4) % WU_XING_NAME.len();

            let g0 = WuXing::new(WU_XING_NAME[i]).unwrap();

            let g1 = WuXing::new(WU_XING_NAME[n0]).unwrap();
            let g2 = WuXing::new(WU_XING_NAME[n1]).unwrap();
            let g3 = WuXing::new(WU_XING_NAME[n2]).unwrap();

            assert!(!g0.sheng(&g1));
            assert!(!g0.sheng(&g2));
            assert!(!g0.sheng(&g3));

            // 自己不生自己
            assert!(!g0.sheng(&g0));
        }
    }

    //五行不相克
    #[test]
    fn test_wu_xing_not_ke() {
        for (i, _) in WU_XING_NAME.iter().enumerate() {
            let n0 = (i + 1) % WU_XING_NAME.len();
            let n1 = (i + 3) % WU_XING_NAME.len();
            let n2 = (i + 4) % WU_XING_NAME.len();

            let g0 = WuXing::new(WU_XING_NAME[i]).unwrap();
            let g1 = WuXing::new(WU_XING_NAME[n0]).unwrap();
            let g2 = WuXing::new(WU_XING_NAME[n1]).unwrap();
            let g3 = WuXing::new(WU_XING_NAME[n2]).unwrap();

            assert!(!g0.ke(&g1));
            assert!(!g0.ke(&g2));
            assert!(!g0.ke(&g3));

            // 自己不克自己
            assert!(!g0.ke(&g0));
        }
    }
}
