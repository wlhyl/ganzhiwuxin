package ganzhiwuxin

import (
	"fmt"
)

var diZhiNumToName = [...]string{"子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"}

type DiZhi struct {
	name   string
	num    int
	wuXing WuXing
	// 阳
	masculine bool
	// 太玄数
	taiXuan int
}

func NewDiZhi(name string) (DiZhi, error) {
	var d DiZhi
	for index, value := range diZhiNumToName {

		if name == value {
			d.name = value
			d.num = index + 1

			d.wuXing = func() WuXing {
				var tmp = (d.num + 10) % 12
				if tmp == 0 {
					tmp = 12
				}
				if tmp%3 == 0 {
					w, _ := NewWuXing("土")
					return w
				} else {
					tmp = tmp/3 + tmp/7 + 1
					w, _ := NewWuXing(wuXinNumToName[tmp-1])
					return w
				}
			}()
			// 阳
			d.masculine = d.num%2 != 0
			// 太玄数
			d.taiXuan = func() int {
				if d.num <= 6 {
					return 10 - d.num
				} else {
					return 16 - d.num
				}
			}()
			return d, nil
		}

	}
	return d, fmt.Errorf("没有此地支：%s", name)
}

func (t DiZhi) Name() string {
	return t.name
}

// func (t DiZhi) GetNum() int {
// 	return t.num
// }

func (d DiZhi) Masculine() bool {
	return d.masculine
}

func (d DiZhi) WuXing() WuXing {
	return d.wuXing
}

func (d DiZhi) TaiXuan() int {
	return d.taiXuan
}

// func (d DiZhi) String() string {
// 	return d.name
// }

func (d DiZhi) Equals(other DiZhi) bool {
	return d.num == other.num
}

func (d DiZhi) Plus(other int) DiZhi {
	var tmp = other
	for tmp < 0 {
		tmp += 12
	}
	tmp = (d.num + tmp) % 12
	if tmp == 0 {
		tmp = 12
	}
	d1, _ := NewDiZhi(diZhiNumToName[tmp-1])
	return d1
}

func (d DiZhi) Minus(other DiZhi) int {
	// 返回整数值
	return (d.num - other.num + 12) % 12
}

func (t DiZhi) Ke(t2 DiZhi) bool {
	return t.wuXing.Ke(t2.WuXing())
}

func (d DiZhi) Sheng(t2 DiZhi) bool {
	return d.wuXing.Sheng(t2.WuXing())
}

//  三合
func (d DiZhi) SanHe(other DiZhi) bool {
	if d.Equals(other) {
		return false
	}
	return (d.num-other.num)%4 == 0
}

// 计算某一地支六合的公式：
// 六合地支 = 丑 - (地支 - 子)
//  六合
func (d DiZhi) LiuHe(other DiZhi) bool {
	if d.Equals(other) {
		return false
	}
	return (d.num+other.num-15)%12 == 0
}

// 六冲
func (d DiZhi) LiuChong(other DiZhi) bool {
	if d.Equals(other) {
		return false
	}
	return (d.num-other.num)%6 == 0
}

// 刑
func (d DiZhi) Xing(other DiZhi) bool {
	if d.name == "子" && other.name == "卯" {
		return true
	}
	if d.name == "丑" && other.name == "戌" {
		return true
	}
	if d.name == "寅" && other.name == "巳" {
		return true
	}
	if d.name == "卯" && other.name == "子" {
		return true
	}
	if d.name == "辰" && other.name == "辰" {
		return true
	}
	if d.name == "巳" && other.name == "申" {
		return true
	}
	if d.name == "午" && other.name == "午" {
		return true
	}
	if d.name == "未" && other.name == "丑" {
		return true
	}
	if d.name == "申" && other.name == "寅" {
		return true
	}
	if d.name == "酉" && other.name == "酉" {
		return true
	}
	if d.name == "戌" && other.name == "未" {
		return true
	}
	if d.name == "亥" && other.name == "亥" {
		return true
	}
	return false
}
