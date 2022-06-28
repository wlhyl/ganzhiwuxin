package ganzhiwuxin

import (
	"testing"
)

var wuXingName = [...]string{"木", "火", "土", "金", "水"}

func TestWuXingNa(t *testing.T) {
	t.Log("Name()方法")
	t.Log("正确创建五行")
	for _, v := range wuXingName {
		w, err := NewWuXing(v)
		if err != nil {
			t.Errorf("初始化五行`%s`失败", v)
		}
		if w.Name() != v {
			t.Error("Name()方法测试失败")
		}
	}

	t.Log("不正确创建五行")
	if _, err := NewWuXing("否"); err == nil {
		t.Error("`否`不是五行")
	}
}

func TestWuXingEquals(t *testing.T) {
	t.Log("五行相等")
	for _, name := range wuXingName {
		w0, err0 := NewWuXing(name)
		w1, err1 := NewWuXing(name)
		if err0 != nil {
			t.Errorf("初始化五行`%s`失败", name)
		}
		if err1 != nil {
			t.Errorf("初始化五行`%s`失败", name)
		}

		if !w0.Equals(w1) {
			t.Error("同一五行相等")
		}
	}
}

func TestWuXingNotEquals(t *testing.T) {
	t.Log(("五行不相等"))
	for i := 0; i < len(wuXingName); i++ {
		w0, err := NewWuXing(wuXingName[i])
		if err != nil {
			t.Error(err)
		}
		for j := 1; j < len(wuXingName)-1; j++ {
			n := (i + j) % len(wuXingName)
			w1, err := NewWuXing(wuXingName[n])
			if err != nil {
				t.Error(err)
			}
			if w0.Equals(w1) {
				t.Error("不同五行不相待")
			}
		}
	}
}

func TestWuXingSheng(t *testing.T) {
	t.Log("五行相生")
	for i, _ := range wuXingName {
		n := (i + 1) % len(wuXingName)
		w0, err0 := NewWuXing(wuXingName[i])
		w1, err1 := NewWuXing(wuXingName[n])
		if err0 != nil {
			t.Error(err0)
		}
		if err1 != nil {
			t.Error(err1)
		}

		if !w0.Sheng(w1) {
			t.Errorf("`%s` 能生 `%s`", w0.Name(), w1.Name())
		}
	}
}

func TestWuXingKe(t *testing.T) {
	t.Log("五行相克")
	for i, _ := range wuXingName {
		n := (i + 2) % len(wuXingName)
		w0, err0 := NewWuXing(wuXingName[i])
		w1, err1 := NewWuXing(wuXingName[n])
		if err0 != nil {
			t.Error(err0)
		}
		if err1 != nil {
			t.Error(err1)
		}
		if !w0.Ke(w1) {
			t.Errorf("`%s` 克 `%s`", w0.Name(), w1.Name())
		}
	}
}

func TestWuXingNotSheng(t *testing.T) {
	for i, _ := range wuXingName {
		n0 := (i + 2) % len(wuXingName)
		n1 := (i + 3) % len(wuXingName)
		n2 := (i + 4) % len(wuXingName)

		g0, err0 := NewWuXing(wuXingName[i])

		g1, err1 := NewWuXing(wuXingName[n0])
		g2, err2 := NewWuXing(wuXingName[n1])
		g3, err3 := NewWuXing(wuXingName[n2])
		if err0 != nil {
			t.Error(err0)
		}
		if err1 != nil {
			t.Error(err1)
		}
		if err2 != nil {
			t.Error(err2)
		}

		if err3 != nil {
			t.Error(err3)
		}

		if g0.Sheng(g1) {
			t.Errorf("`%s` 不生 `%s`", g0.Name(), g1.Name())
		}
		if g0.Sheng(g2) {
			t.Errorf("`%s` 不生 `%s`", g0.Name(), g2.Name())
		}
		if g0.Sheng(g3) {
			t.Errorf("`%s` 不生 `%s`", g0.Name(), g3.Name())
		}

		// 自己不生自己
		if g0.Sheng(g0) {
			t.Errorf("`%s` 不生 `%s`", g0.Name(), g0.Name())
		}
	}
}

func TestWuXingNotKe(t *testing.T) {
	t.Log("五行不相克")
	for i, _ := range wuXingName {
		n0 := (i + 1) % len(wuXingName)
		n1 := (i + 3) % len(wuXingName)
		n2 := (i + 4) % len(wuXingName)

		g0, err0 := NewWuXing(wuXingName[i])
		g1, err1 := NewWuXing(wuXingName[n0])
		g2, err2 := NewWuXing(wuXingName[n1])
		g3, err3 := NewWuXing(wuXingName[n2])

		if err0 != nil {
			t.Error(err0)
		}
		if err1 != nil {
			t.Error(err1)
		}
		if err2 != nil {
			t.Error(err2)
		}

		if err3 != nil {
			t.Error(err3)
		}

		if g0.Ke(g1) {
			t.Errorf("`%s` 不克 `%s`", g0.Name(), g1.Name())
		}
		if g0.Ke(g2) {
			t.Errorf("`%s` 不克 `%s`", g0.Name(), g2.Name())
		}
		if g0.Ke(g3) {
			t.Errorf("`%s` 不克 `%s`", g0.Name(), g3.Name())
		}

		// 自己不克自己
		if g0.Ke(g0) {
			t.Errorf("`%s` 不克 `%s`", g0.Name(), g0.Name())
		}
	}
}
