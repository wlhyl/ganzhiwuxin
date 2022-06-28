package ganzhiwuxin

import "testing"

var tianGanName = [...]string{"甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"}

func TestNewTianGang(t *testing.T) {
	t.Log("NewTianGang()方法")
	t.Log("正确创建TianGang")
	for _, v := range tianGanName {
		w, err := NewTianGan(v)
		if err != nil {
			t.Errorf("初始天干`%s`失败", v)
		}
		if w.Name() != v {
			t.Error("Name()方法测试失败")
		}
	}

	t.Log("不正确创建天干")
	if _, err := NewTianGan("否"); err == nil {
		t.Error("`否`不是天干")
	}
}

func TestTianGanWuXing(t *testing.T) {
	t.Log("天干的五行")
	w, _ := NewWuXing("木")
	tg, _ := NewTianGan("甲")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("木")
	tg, _ = NewTianGan("乙")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("火")
	tg, _ = NewTianGan("丙")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("火")
	tg, _ = NewTianGan("丁")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("土")
	tg, _ = NewTianGan("戊")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("土")
	tg, _ = NewTianGan("己")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("金")
	tg, _ = NewTianGan("庚")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("金")
	tg, _ = NewTianGan("辛")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("水")
	tg, _ = NewTianGan("壬")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
	w, _ = NewWuXing("水")
	tg, _ = NewTianGan("癸")
	if !w.Equals(tg.WuXing()) {
		t.Errorf("%s 的五行是 %s", t.Name(), w.Name())
	}
}

func TestTianGangMasculine(t *testing.T) {
	t.Log("天干的阴阳")
	for i, name := range tianGanName {
		tg, _ := NewTianGan(name)
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

func TestTianGanTaiXuan(t *testing.T) {
	t.Log("天干的太玄数")

	n := 9
	tg, _ := NewTianGan("甲")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 8
	tg, _ = NewTianGan("乙")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 7
	tg, _ = NewTianGan("丙")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 6
	tg, _ = NewTianGan("丁")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 5
	tg, _ = NewTianGan("戊")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 9
	tg, _ = NewTianGan("己")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 8
	tg, _ = NewTianGan("庚")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 7
	tg, _ = NewTianGan("辛")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 6
	tg, _ = NewTianGan("壬")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
	n = 5
	tg, _ = NewTianGan("癸")
	if n != tg.TaiXuan() {
		t.Errorf("%s 的太玄数是 %v", tg.Name(), n)
	}
}

func TestTianGanEquals(t *testing.T) {
	t.Log("天干相等")
	for _, name := range tianGanName {
		tg0, _ := NewTianGan(name)
		tg1, _ := NewTianGan(name)
		if !tg0.Equals(tg1) {
			t.Errorf("%s 与 %s 相等", tg0.Name(), tg1.Name())
		}
	}
}

func TestTianGanNotEqual(t *testing.T) {
	t.Log("天干不相等")
	for i, _ := range tianGanName {
		g0, _ := NewTianGan(tianGanName[i])
		for j := 1; j < len(tianGanName); j++ {
			n := (i + j) % len(tianGanName)
			g1, _ := NewTianGan(tianGanName[n])
			if g0.Equals(g1) {
				t.Errorf("%s != %s", g0.Name(), g1.Name())
			}
		}
	}
}

//用数学归纳法
func TestTianGanPlus(t *testing.T) {
	t.Log("天干 + 整数")
	for i, _ := range tianGanName {
		tg0, _ := NewTianGan(tianGanName[(i+1)%len(tianGanName)])
		tg1, _ := NewTianGan(tianGanName[i])
		tg1 = tg1.Plus(1)
		if !tg0.Equals(tg1) {
			t.Errorf("%s + 1 = %s", tg1.Name(), tg0.Name())
		}
	}

	for i := range tianGanName {
		tg0, _ := NewTianGan(tianGanName[(i-1+len(tianGanName))%len(tianGanName)])
		tg1, _ := NewTianGan(tianGanName[i])
		tg1 = tg1.Plus(-1)
		if !tg0.Equals(tg1) {
			t.Errorf("%s + (-1) = %s", tg1.Name(), tg0.Name())
		}
	}

	for _, it := range tianGanName {
		g, _ := NewTianGan(it)
		if !(g.Plus(99)).Plus(1).Equals(g.Plus(100)) {
			t.Errorf("%s + 99 + 1 = %s + 100", g.Name(), g.Name())
		}
		if !g.Plus(-99).Plus(-1).Equals(g.Plus(-100)) {
			t.Errorf("%s + (-99) + (-1) = %s + (-100)", g.Name(), g.Name())
		}
	}
}

func TestTianGanMinus(t *testing.T) {
	t.Log("天干 - 天干 = 正整数")
	for i, _ := range tianGanName {
		g0, _ := NewTianGan(tianGanName[i])
		for j, _ := range tianGanName {
			g1 := g0.Plus(j)
			if j != g1.Minus(g0) {
				t.Errorf("%s - %s = %v", g1.Name(), g0.Name(), j)
			}
		}
	}
}

func TestTianGanKe(t *testing.T) {
	t.Log("天干相克")
	for i, _ := range tianGanName {
		g0, _ := NewTianGan(tianGanName[i])
		for j, _ := range tianGanName {
			g1, _ := NewTianGan(tianGanName[j])
			if g0.WuXing().Ke(g1.WuXing()) != g0.Ke(g1) {
				t.Errorf("%s 克 %s", g0.Name(), g1.Name())
			}
		}
	}
}

func TestTianGanSheng(t *testing.T) {
	t.Log("天干相生")
	for i, _ := range tianGanName {
		g0, _ := NewTianGan(tianGanName[i])
		for j, _ := range tianGanName {
			g1, _ := NewTianGan(tianGanName[j])
			if g0.WuXing().Sheng(g1.WuXing()) != g0.Sheng(g1) {
				t.Errorf("%s 生 %s", g0.Name(), g1.Name())
			}
		}
	}
}

func TestWuHe(t *testing.T) {
	t.Log("天干五合")
	for _, it := range tianGanName {
		g0, _ := NewTianGan(it)
		for i := 0; i < 10; i++ {
			g1 := g0.Plus(i)
			if i == 5 {
				if !g0.WuHe(g1) {
					t.Errorf("%s 五合 %s", g0.Name(), g1.Name())
				}
			} else {
				if g0.WuHe(g1) {
					t.Errorf("%s 不五合 %s", g0.Name(), g1.Name())
				}
			}
		}

	}
}
