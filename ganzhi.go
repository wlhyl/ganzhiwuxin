package ganzhiwuxin

import "fmt"

type GanZhi struct {
	gan TianGan
	zhi DiZhi
	// 纳音
	naYin WuXing
}

func NewGanZhi(gan TianGan, zhi DiZhi) (GanZhi, error) {
	var gz GanZhi
	if gan.masculine != zhi.masculine {
		return gz, fmt.Errorf("干支阴阳不相同")
	}
	gz.gan = gan
	gz.zhi = zhi
	wuXingName := [...]string{"木", "火", "土", "金", "水"}
	var g TianGan
	if gan.masculine {
		g = gan.Plus(1)
	} else {
		g = gan.Plus(-1)
	}
	var z DiZhi
	if zhi.masculine {
		z = zhi.Plus(1)
	} else {
		z = zhi.Plus(-1)
	}
	var n = (49 - (gan.taiXuan + zhi.taiXuan + g.taiXuan + z.taiXuan)) % 10
	w := func() WuXing {
		if n == 1 || n == 6 {
			w, _ := NewWuXing("水")
			return w
		}
		if n == 2 || n == 7 {
			w, _ := NewWuXing("火")
			return w
		}
		if n == 3 || n == 8 {
			w, _ := NewWuXing("木")
			return w
		}
		if n == 4 || n == 9 {
			w, _ := NewWuXing("金")
			return w
		}
		w, _ := NewWuXing("土")
		return w
	}()
	// n = (wuXingName.indexOf(w.toString()) + 1) % 5
	n = w.num % 5
	gz.naYin = WuXing{wuXingName[n], n + 1}
	return gz, nil
}

func (gz GanZhi) Name() string {
	return fmt.Sprintf("%s%s", gz.gan.name, gz.zhi.name)
}

func (gz GanZhi) Gan() TianGan {
	return gz.gan
}

func (gz GanZhi) Zhi() DiZhi {
	return gz.zhi
}

func (gz GanZhi) Equals(other GanZhi) bool {
	return gz.gan.Equals(other.gan) && gz.zhi.Equals(other.zhi)
}

func (gz GanZhi) Plus(other int) GanZhi {
	g := gz.gan.Plus(other)
	z := gz.zhi.Plus(other)
	gz0, _ := NewGanZhi(g, z)
	return gz0
}

func (gz GanZhi) Minus(other GanZhi) int {
	// 返回整数值
	return (gz.num() - other.num() + 60) % 60
}

func (gz GanZhi) num() int {
	g, _ := NewTianGan("甲")
	z, _ := NewDiZhi("子")
	var n int = 0
	for i := 0; i < 60; i++ {
		if g.Plus(i).Equals(gz.gan) && z.Plus(i).Equals(gz.zhi) {
			n = i
			break
		}
	}
	return n + 1
}
