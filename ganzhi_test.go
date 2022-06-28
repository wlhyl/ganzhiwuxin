package ganzhiwuxin

import (
	"fmt"
	"testing"
)

// private val naYing = listOf(
//     NaYingData("甲子", WuXing("金")),
//     NaYingData("乙丑", WuXing("金")),
//     NaYingData("丙寅", WuXing("火")),
//     NaYingData("丁卯", WuXing("火")),
//     NaYingData("戊辰", WuXing("木")),
//     NaYingData("己巳", WuXing("木")),
//     NaYingData("庚午", WuXing("土")),
//     NaYingData("辛未", WuXing("土")),
//     NaYingData("壬申", WuXing("金")),
//     NaYingData("癸酉", WuXing("金")),
//     NaYingData("甲戌", WuXing("火")),
//     NaYingData("乙亥", WuXing("火")),
//     NaYingData("丙子", WuXing("水")),
//     NaYingData("丁丑", WuXing("水")),
//     NaYingData("戊寅", WuXing("土")),
//     NaYingData("己卯", WuXing("土")),
//     NaYingData("庚辰", WuXing("金")),
//     NaYingData("辛巳", WuXing("金")),
//     NaYingData("壬午", WuXing("木")),
//     NaYingData("癸未", WuXing("木")),
//     NaYingData("壬午", WuXing("木")),
//     NaYingData("癸未", WuXing("木")),
//     NaYingData("甲申", WuXing("水")),
//     NaYingData("乙酉", WuXing("水")),
//     NaYingData("丙戌", WuXing("土")),
//     NaYingData("丁亥", WuXing("土")),
//     NaYingData("戊子", WuXing("火")),
//     NaYingData("己丑", WuXing("火")),
//     NaYingData("庚寅", WuXing("木")),
//     NaYingData("辛卯", WuXing("木")),
//     NaYingData("壬辰", WuXing("水")),
//     NaYingData("癸巳", WuXing("水")),
//     NaYingData("甲午", WuXing("金")),
//     NaYingData("乙未", WuXing("金")),
//     NaYingData("丙申", WuXing("火")),
//     NaYingData("丁酉", WuXing("火")),
//     NaYingData("戊戌", WuXing("木")),
//     NaYingData("己亥", WuXing("木")),
//     NaYingData("庚子", WuXing("土")),
//     NaYingData("辛丑", WuXing("土")),
//     NaYingData("壬寅", WuXing("金")),
//     NaYingData("癸卯", WuXing("金")),
//     NaYingData("甲辰", WuXing("火")),
//     NaYingData("乙巳", WuXing("火")),
//     NaYingData("丙午", WuXing("水")),
//     NaYingData("丁未", WuXing("水")),
//     NaYingData("戊申", WuXing("土")),
//     NaYingData("己酉", WuXing("土")),
//     NaYingData("庚戌", WuXing("金")),
//     NaYingData("辛亥", WuXing("金")),
//     NaYingData("壬子", WuXing("木")),
//     NaYingData("癸丑", WuXing("木")),
//     NaYingData("甲寅", WuXing("水")),
//     NaYingData("乙卯", WuXing("水")),
//     NaYingData("丙辰", WuXing("土")),
//     NaYingData("丁巳", WuXing("土")),
//     NaYingData("戊午", WuXing("火")),
//     NaYingData("己未", WuXing("火")),
//     NaYingData("庚申", WuXing("木")),
//     NaYingData("辛酉", WuXing("木")),
//     NaYingData("壬戌", WuXing("水")),
//     NaYingData("癸亥", WuXing("水")),
// )

var naYing = [...]string{
	"金",
	"金",
	"火",
	"火",
	"木",
	"木",
	"土",
	"土",
	"金",
	"金",
	"火",
	"火",
	"水",
	"水",
	"土",
	"土",
	"金",
	"金",
	"木",
	"木",
	"水",
	"水",
	"土",
	"土",
	"火",
	"火",
	"木",
	"木",
	"水",
	"水",
	"金",
	"金",
	"火",
	"火",
	"木",
	"木",
	"土",
	"土",
	"金",
	"金",
	"火",
	"火",
	"水",
	"水",
	"土",
	"土",
	"金",
	"金",
	"木",
	"木",
	"水",
	"水",
	"土",
	"土",
	"火",
	"火",
	"木",
	"木",
	"水",
	"水",
}

func TestNewGanZhi(t *testing.T) {
	t.Log("测试NewGanZhi()方法")
	jia, _ := NewTianGan("甲")
	zi, _ := NewDiZhi("子")
	for i := 0; i < 10; i++ {
		g := jia.Plus(i)
		for j := 0; j < 12; j++ {
			z := zi.Plus(j)
			gz, err := NewGanZhi(g, z)
			if g.masculine == z.masculine {
				if err != nil {
					t.Fatalf("NewGanZhi()方法错误, %s%s是正确的干支", g.name, z.name)
				}
				if gz.Name() != fmt.Sprintf("%s%s", g.name, z.name) {
					t.Fatalf("Name()方法测试失败")
				}
			} else {
				if err == nil {
					t.Fatalf("NewGanZhi()方法错误, %s%s不是正确的干支", g.name, z.name)
				}
			}
		}
	}
}

var ganZhiName = [...]string{
	"甲子",
	"乙丑",
	"丙寅",
	"丁卯",
	"戊辰",
	"己巳",
	"庚午",
	"辛未",
	"壬申",
	"癸酉",
	"甲戌",
	"乙亥",
	"丙子",
	"丁丑",
	"戊寅",
	"己卯",
	"庚辰",
	"辛巳",
	"壬午",
	"癸未",
	"甲申",
	"乙酉",
	"丙戌",
	"丁亥",
	"戊子",
	"己丑",
	"庚寅",
	"辛卯",
	"壬辰",
	"癸巳",
	"甲午",
	"乙未",
	"丙申",
	"丁酉",
	"戊戌",
	"己亥",
	"庚子",
	"辛丑",
	"壬寅",
	"癸卯",
	"甲辰",
	"乙巳",
	"丙午",
	"丁未",
	"戊申",
	"己酉",
	"庚戌",
	"辛亥",
	"壬子",
	"癸丑",
	"甲寅",
	"乙卯",
	"丙辰",
	"丁巳",
	"戊午",
	"己未",
	"庚申",
	"辛酉",
	"壬戌",
	"癸亥",
}

func TestGanAndZhi(t *testing.T) {
	t.Log("测试Gan(), Zhi()方法")
	jia, _ := NewTianGan("甲")
	zi, _ := NewDiZhi("子")
	for i := 0; i < 10; i++ {
		g := jia.Plus(i)
		for j := 0; j < 12; j++ {
			z := zi.Plus(j)
			if g.masculine != z.masculine {
				continue
			}
			gz, _ := NewGanZhi(g, z)
			if !gz.gan.Equals(g) {
				t.Fatalf("%s的干是%s, 非是%s", gz.Name(), g.Name(), gz.Gan().name)
			}
			if !gz.zhi.Equals(z) {
				t.Fatalf("%s的支是%s, 非是%s", gz.Name(), z.Name(), gz.Zhi().name)
			}
		}
	}
}

func TestGanZhiEquals(t *testing.T) {
	t.Log("干支相等")
	for _, name := range ganZhiName {
		gzSring := []rune(name)
		g, _ := NewTianGan(string(gzSring[0]))
		z, _ := NewDiZhi(string(gzSring[1]))
		gz0, _ := NewGanZhi(g, z)
		gz1, _ := NewGanZhi(g, z)

		if !gz0.Equals(gz1) {
			t.Errorf("%s 与 %s 相等", gz0.Name(), gz1.Name())
		}
	}
}

func TestGanZhiNotEquals(t *testing.T) {
	t.Log("干支不相等")
	for i, name := range ganZhiName {
		gzSring0 := []rune(name)
		g0, _ := NewTianGan(string(gzSring0[0]))
		z0, _ := NewDiZhi(string(gzSring0[1]))
		gz0, _ := NewGanZhi(g0, z0)

		for j := 1; j < 60; j++ {
			n := (i + j) % 60
			gzSring1 := []rune(ganZhiName[n])
			g1, _ := NewTianGan(string(gzSring1[0]))
			z1, _ := NewDiZhi(string(gzSring1[1]))
			gz1, _ := NewGanZhi(g1, z1)
			if gz0.Equals(gz1) {
				t.Errorf("%s != %s", gz0.Name(), gz1.Name())
			}
		}
	}
}

//用数学归纳法
func TestGanZhiPlus(t *testing.T) {
	t.Log("干支 + 整数")
	for i, name := range ganZhiName {
		gzSring0 := []rune(ganZhiName[(i+1)%60])
		g0, _ := NewTianGan(string(gzSring0[0]))
		z0, _ := NewDiZhi(string(gzSring0[1]))
		gz0, _ := NewGanZhi(g0, z0)

		gzSring0 = []rune(name)
		g0, _ = NewTianGan(string(gzSring0[0]))
		z0, _ = NewDiZhi(string(gzSring0[1]))
		gz1, _ := NewGanZhi(g0, z0)
		gz1 = gz1.Plus(1)
		if !gz0.Equals(gz1) {
			t.Errorf("%s + 1 = %s", gz1.Name(), gz0.Name())
		}
	}

	for i, name := range ganZhiName {
		gzSring0 := []rune(ganZhiName[(i-1+60)%60])
		g0, _ := NewTianGan(string(gzSring0[0]))
		z0, _ := NewDiZhi(string(gzSring0[1]))
		gz0, _ := NewGanZhi(g0, z0)

		gzSring0 = []rune(name)
		g0, _ = NewTianGan(string(gzSring0[0]))
		z0, _ = NewDiZhi(string(gzSring0[1]))
		gz1, _ := NewGanZhi(g0, z0)
		gz1 = gz1.Plus(-1)

		if !gz0.Equals(gz1) {
			t.Errorf("%s + (-1) = %s", gz1.Name(), gz0.Name())
		}
	}
	for _, name := range ganZhiName {
		gzSring0 := []rune(name)
		g0, _ := NewTianGan(string(gzSring0[0]))
		z0, _ := NewDiZhi(string(gzSring0[1]))
		gz, _ := NewGanZhi(g0, z0)

		if !(gz.Plus(99)).Plus(1).Equals(gz.Plus(100)) {
			t.Errorf("%s + 99 + 1 = %s + 100", gz.Name(), gz.Name())
		}
		if !gz.Plus(-99).Plus(-1).Equals(gz.Plus(-100)) {
			t.Errorf("%s + (-99) + (-1) = %s + (-100)", gz.Name(), gz.Name())
		}
	}
}

func TestGanZhiMinus(t *testing.T) {
	t.Log("干支 - 干支 = 正整数")
	for _, name := range ganZhiName {
		gzSring0 := []rune(name)
		g0, _ := NewTianGan(string(gzSring0[0]))
		z0, _ := NewDiZhi(string(gzSring0[1]))
		gz0, _ := NewGanZhi(g0, z0)

		for j, _ := range ganZhiName {
			gz1 := gz0.Plus(j)
			if j != gz1.Minus(gz0) {
				t.Errorf("%s - %s = %v", gz1.Name(), gz0.Name(), j)
			}
		}
	}
}

func TestNaYin(t *testing.T) {
	t.Log("测试60甲子纳音")
	g, _ := NewTianGan("甲")
	z, _ := NewDiZhi("子")
	gz, _ := NewGanZhi(g, z)
	for i := 0; i < 60; i++ {
		gz0 := gz.Plus(i)
		w := naYing[i]
		if gz0.naYin.name != w {
			t.Fatalf("%s 的纳音是 %s, 非是%s", gz0.Name(), w, gz0.naYin.name)
		}
	}
}
