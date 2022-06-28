package ganzhiwuxin

import (
	"fmt"
)

var tianGanNumToName = [...]string{"甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"}

type TianGan struct {
	name   string
	num    int
	wuXing WuXing
	//  阳
	masculine bool
	// 太玄数
	taiXuan int
}

func NewTianGan(name string) (TianGan, error) {
	var t TianGan
	for index, value := range tianGanNumToName {
		if name == value {
			t.name = value
			t.num = index + 1

			t.wuXing.num = (t.num + 1) / 2
			t.wuXing.name = wuXinNumToName[t.wuXing.num-1]

			t.masculine = t.num%2 != 0

			if t.num <= 5 {
				t.taiXuan = 10 - t.num
			} else {
				t.taiXuan = 15 - t.num
			}

			return t, nil
		}
	}

	return t, fmt.Errorf("没有此天干：%s", name)

}

func (t TianGan) Name() string {
	return t.name
}

// 阴阳
func (t TianGan) Masculine() bool {
	return t.masculine
}

//返回干的五行
func (t TianGan) WuXing() WuXing {
	return t.wuXing
}

// 太玄数
func (t TianGan) TaiXuan() int {
	return t.taiXuan
}

// 相等
func (t TianGan) Equals(other TianGan) bool {
	return t.num == other.num
}

// 天干向前数n个
func (t TianGan) Plus(other int) TianGan {
	var tmp = other
	for tmp < 0 {
		tmp += 10
	}
	tmp = (t.num + tmp + 10) % 10
	if tmp == 0 {
		tmp = 10
	}
	t1, _ := NewTianGan(tianGanNumToName[tmp-1])
	return t1
}

// 两天干相差几位
func (t TianGan) Minus(other TianGan) int {
	// 返回值为整数
	return (t.num - other.num + 10) % 10
}

//  克
func (t TianGan) Ke(other TianGan) bool {
	return t.wuXing.Ke(other.wuXing)
}

//  生
func (t TianGan) Sheng(other TianGan) bool {
	return t.wuXing.Sheng(other.wuXing)
}

// 五合
func (t TianGan) WuHe(other TianGan) bool {
	if t.num == other.num {
		return false
	}
	return (t.num-other.num)%5 == 0
}
