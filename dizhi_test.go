package ganzhiwuxin

import "testing"

var diZhiName = [...]string{"子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"}

func TestNewDiZhi(t *testing.T) {
	t.Log("NewDiZhi()方法")
	t.Log("正确创建DiZhi")
	for _, v := range diZhiName {
		w, err := NewDiZhi(v)
		if err != nil {
			t.Error(err)
		}
		if w.Name() != v {
			t.Error("Name()方法测试失败")
		}
	}

	t.Log("不正确地支创建")
	if _, err := NewDiZhi("否"); err == nil {
		t.Error("`否`不是地支")
	}
}

func TestDiZhiWuXing(t *testing.T) {
	t.Log("地支五行")
	w, _ := NewWuXing("水")
	tg, _ := NewDiZhi("子")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("土")
	tg, _ = NewDiZhi("丑")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("木")
	tg, _ = NewDiZhi("寅")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("木")
	tg, _ = NewDiZhi("卯")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("土")
	tg, _ = NewDiZhi("辰")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("火")
	tg, _ = NewDiZhi("巳")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("火")
	tg, _ = NewDiZhi("午")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("土")
	tg, _ = NewDiZhi("未")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("金")
	tg, _ = NewDiZhi("申")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("金")
	tg, _ = NewDiZhi("酉")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}

	w, _ = NewWuXing("土")
	tg, _ = NewDiZhi("戌")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}

	w, _ = NewWuXing("水")
	tg, _ = NewDiZhi("亥")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
}

func TestDiZhiMasculine(t *testing.T) {
	t.Log("地支的阴阳")
	for i, name := range diZhiName {
		tg, _ := NewDiZhi(name)
		if i%2 == 0 {
			if !tg.Masculine() {
				t.Errorf("%s 是阳", tg.Name())
			}
		} else {
			if tg.Masculine() {
				t.Errorf("%s 是阴", tg.Name())
			}
		}
	}
}

func TestDiZhiTaiXuan(t *testing.T) {
	t.Log("地支的太玄数")

	n := 9
	tg, _ := NewDiZhi("子")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	tg, _ = NewDiZhi("午")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}

	n = 8
	tg, _ = NewDiZhi("丑")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	tg, _ = NewDiZhi("未")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}

	n = 7
	tg, _ = NewDiZhi("寅")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	tg, _ = NewDiZhi("申")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}

	n = 6
	tg, _ = NewDiZhi("卯")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	tg, _ = NewDiZhi("酉")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}

	n = 5
	tg, _ = NewDiZhi("辰")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	tg, _ = NewDiZhi("戌")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
}

func TestDiZhiEquals(t *testing.T) {
	t.Log("地支相等")
	for _, name := range diZhiName {
		tg0, _ := NewDiZhi(name)
		tg1, _ := NewDiZhi(name)
		if !tg0.Equals(tg1) {
			t.Errorf("%s 与 %s 相等", tg0.Name(), tg1.Name())
		}
	}
}

func TestDiZhiNotEqual(t *testing.T) {
	t.Log("地支不相等")
	for i, _ := range diZhiName {
		g0, _ := NewDiZhi(diZhiName[i])
		for j := 1; j < len(diZhiName); j++ {
			n := (i + j) % len(diZhiName)
			g1, _ := NewDiZhi(diZhiName[n])
			if g0.Equals(g1) {
				t.Errorf("%s != %s", g0.Name(), g1.Name())
			}
		}
	}
}

//用数学归纳法
func TestDiZhiPlus(t *testing.T) {
	t.Log("地支 + 整数")
	for i, _ := range diZhiName {
		tg0, _ := NewDiZhi(diZhiName[(i+1)%len(diZhiName)])
		tg1, _ := NewDiZhi(diZhiName[i])
		tg1 = tg1.Plus(1)
		if !tg0.Equals(tg1) {
			t.Errorf("%s + 1 = %s", tg1.Name(), tg0.Name())
		}
	}

	for i, _ := range diZhiName {
		tg0, _ := NewDiZhi(diZhiName[(i-1+len(diZhiName))%len(diZhiName)])
		tg1, _ := NewDiZhi(diZhiName[i])
		tg1 = tg1.Plus(-1)
		if !tg0.Equals(tg1) {
			t.Errorf("%s + (-1) = %s", tg1.Name(), tg0.Name())
		}
	}

	for _, it := range diZhiName {
		g, _ := NewDiZhi(it)
		if !(g.Plus(99)).Plus(1).Equals(g.Plus(100)) {
			t.Errorf("%s + 99 + 1 = %s + 100", g.Name(), g.Name())
		}
		if !g.Plus(-99).Plus(-1).Equals(g.Plus(-100)) {
			t.Errorf("%s + (-99) + (-1) = %s + (-100)", g.Name(), g.Name())
		}
	}
}

func TestDiZhiMinus(t *testing.T) {
	t.Log("地支 - 地支 = 正整数")
	for i, _ := range diZhiName {
		g0, _ := NewDiZhi(diZhiName[i])
		for j, _ := range diZhiName {
			g1 := g0.Plus(j)
			if j != g1.Minus(g0) {
				t.Errorf("%s - %s = %v", g1.Name(), g0.Name(), j)
			}
		}
	}
}

func TestDiZhiKe(t *testing.T) {
	t.Log("地支相克")
	for i, _ := range diZhiName {
		g0, _ := NewDiZhi(diZhiName[i])
		for j, _ := range diZhiName {
			g1, _ := NewDiZhi(diZhiName[j])
			if g0.WuXing().Ke(g1.WuXing()) != g0.Ke(g1) {
				t.Errorf("%s 克 %s", g0.Name(), g1.Name())
			}
		}
	}
}

func TestDiZhiSheng(t *testing.T) {
	t.Log("地支相生")
	for i, _ := range diZhiName {
		g0, _ := NewDiZhi(diZhiName[i])
		for j, _ := range diZhiName {
			g1, _ := NewDiZhi(diZhiName[j])
			if g0.WuXing().Sheng(g1.WuXing()) != g0.Sheng(g1) {
				t.Errorf("%s 生 %s", g0.Name(), g1.Name())
			}
		}
	}
}

func TestSanHe(t *testing.T) {
	t.Log("地支三合")
	for _, it := range diZhiName {
		d0, _ := NewDiZhi(it)
		d1 := d0.Plus(4)
		d2 := d1.Plus(4)
		if !d0.SanHe(d1) {
			t.Errorf("%s 三合 %s", d0.Name(), d1.Name())
		}
		if !d0.SanHe(d2) {
			t.Errorf("%s 三合 %s", d0.Name(), d2.Name())
		}
		for _, i := range diZhiName {
			d, _ := NewDiZhi(i)
			if (!d.Equals(d1)) && (!d.Equals(d2)) {
				if d0.SanHe(d) {
					t.Errorf("%s 不五合 %s", d0.Name(), d.Name())
				}
			}
		}
	}
}

// 计算某一地支六合的公式：
// 六合地支 = 丑 - (地支 - 子)
func TestLiuHe(t *testing.T) {
	t.Log("测试六合")
	zi, _ := NewDiZhi("子")
	chou, _ := NewDiZhi("丑")
	for _, name := range diZhiName {
		d0, _ := NewDiZhi(name)
		n := d0.Minus(zi)
		d1 := chou.Plus(-n)
		if !d0.LiuHe(d1) {
			t.Fatalf("%s 六合 %s", d0.Name(), d1.Name())
		}

		for _, j := range diZhiName {
			d, _ := NewDiZhi(j)
			if !d.Equals(d1) {
				if d0.LiuHe(d) {
					t.Fatalf("%s 不六合 %s", d0.Name(), d.Name())
				}
			}
		}
	}
}

func TestLiuChong(t *testing.T) {
	t.Log("测试六冲")
	for _, it := range diZhiName {
		d0, _ := NewDiZhi(it)
		d1 := d0.Plus(6)
		if !d0.LiuChong(d1) {
			t.Fatalf("%s 六冲 %s", d0.Name(), d1.Name())
		}
		for _, i := range diZhiName {
			d, _ := NewDiZhi(i)
			if !d.Equals(d1) {
				if d0.LiuChong(d) {
					t.Fatalf("%s 不六冲 %s", d0.Name(), d.Name())
				}
			}
		}
	}
}

func TestXing(t *testing.T) {
	xing := [...]string{"卯", "戌", "巳", "子", "辰", "申", "午", "丑", "寅", "酉", "未", "亥"}
	for index, name := range diZhiName {
		d0, _ := NewDiZhi(name)
		d1, _ := NewDiZhi(xing[index])
		if !d0.Xing(d1) {
			t.Fatalf("%s 刑 %s", d0.Name(), d1.Name())
		}
		for _, j := range diZhiName {
			d, _ := NewDiZhi(j)
			if (!d.Equals(d1)) && d0.Xing(d) {
				t.Fatalf("%s 不刑 %s", d0.Name(), d.Name())
			}
		}
	}
}
