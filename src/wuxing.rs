use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WuXing {
    木,
    火,
    土,
    金,
    水,
}

impl WuXing {
    fn num(&self) -> u8 {
        match self {
            WuXing::木 => 1,
            WuXing::火 => 2,
            WuXing::土 => 3,
            WuXing::金 => 4,
            WuXing::水 => 5,
        }
    }
    /// 生
    pub fn sheng(&self, w2: &WuXing) -> bool {
        return (self.num() as i16 - w2.num() as i16 - 4) % 5 == 0;
    }

    /// 克
    pub fn ke(&self, w2: &WuXing) -> bool {
        return (self.num() as i16 - w2.num() as i16 - 3) % 5 == 0;
    }
}

impl Display for WuXing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WuXing::木 => write!(f, "{}", "木"),
            WuXing::火 => write!(f, "{}", "火"),
            WuXing::土 => write!(f, "{}", "土"),
            WuXing::金 => write!(f, "{}", "金"),
            WuXing::水 => write!(f, "{}", "水"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WuXing;

    const WU_XING: [WuXing; 5] = [WuXing::木, WuXing::火, WuXing::土, WuXing::金, WuXing::水];

    #[test]
    fn test_num() {
        assert_eq!(WuXing::木.num(), 1);
        assert_eq!(WuXing::火.num(), 2);
        assert_eq!(WuXing::土.num(), 3);
        assert_eq!(WuXing::金.num(), 4);
        assert_eq!(WuXing::水.num(), 5);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", WuXing::木), "木");
        assert_eq!(format!("{}", WuXing::火), "火");
        assert_eq!(format!("{}", WuXing::土), "土");
        assert_eq!(format!("{}", WuXing::金), "金");
        assert_eq!(format!("{}", WuXing::水), "水");
    }
    #[test]
    fn test_wu_xing_equals() {
        assert_eq!(WuXing::木, WuXing::木);
        assert_eq!(WuXing::火, WuXing::火);
        assert_eq!(WuXing::土, WuXing::土);
        assert_eq!(WuXing::金, WuXing::金);
        assert_eq!(WuXing::水, WuXing::水);
    }

    #[test]
    fn test_wu_xing_not_equals() {
        for i in 0..WU_XING.len() {
            let w0 = WU_XING[i].clone();

            for j in 1..WU_XING.len() - 1 {
                let n = (i + j) % WU_XING.len();
                let w1 = WU_XING[n].clone();

                assert_ne!(w0, w1);
                assert_ne!(w1, w0);
            }
        }
    }

    // 五行相生
    #[test]
    fn test_wu_xing_sheng() {
        for (i, _) in WU_XING.iter().enumerate() {
            let n = (i + 1) % WU_XING.len();
            let w0 = WU_XING[i].clone();
            let w1 = WU_XING[n].clone();

            assert!(w0.sheng(&w1));
        }
    }

    // 五行相克
    #[test]
    fn test_wu_xing_ke() {
        for (i, _) in WU_XING.iter().enumerate() {
            let n = (i + 2) % WU_XING.len();
            let w0 = WU_XING[i].clone();
            let w1 = WU_XING[n].clone();

            assert!(w0.ke(&w1));
        }
    }

    #[test]
    fn test_wu_xing_not_sheng() {
        for (i, _) in WU_XING.iter().enumerate() {
            let n0 = (i + 2) % WU_XING.len();
            let n1 = (i + 3) % WU_XING.len();
            let n2 = (i + 4) % WU_XING.len();

            let g0 = WU_XING[i].clone();

            let g1 = WU_XING[n0].clone();
            let g2 = WU_XING[n1].clone();
            let g3 = WU_XING[n2].clone();

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
        for (i, _) in WU_XING.iter().enumerate() {
            let n0 = (i + 1) % WU_XING.len();
            let n1 = (i + 3) % WU_XING.len();
            let n2 = (i + 4) % WU_XING.len();

            let g0 = WU_XING[i].clone();
            let g1 = WU_XING[n0].clone();
            let g2 = WU_XING[n1].clone();
            let g3 = WU_XING[n2].clone();

            assert!(!g0.ke(&g1));
            assert!(!g0.ke(&g2));
            assert!(!g0.ke(&g3));

            // 自己不克自己
            assert!(!g0.ke(&g0));
        }
    }
}
