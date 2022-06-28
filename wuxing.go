package ganzhiwuxin

import "fmt"

var wuXinNumToName = [...]string{"木", "火", "土", "金", "水"}

type WuXing struct {
	name string
	num  int
}

func NewWuXing(name string) (WuXing, error) {
	var w WuXing
	for index, value := range wuXinNumToName {

		if name == value {
			w.name = value
			w.num = index + 1
			return w, nil
		}
	}
	return w, fmt.Errorf("没有此五行：%s", name)
}

/**
得到五行的名字
*/
func (w WuXing) Name() string {
	return w.name
}

/**
相同判断
*/
func (w WuXing) Equals(other WuXing) bool {
	return w.num == other.num
}

// 生
func (w WuXing) Sheng(w2 WuXing) bool {
	return (w.num-w2.num-4)%5 == 0
}

// 克
func (w WuXing) Ke(w2 WuXing) bool {
	return (w.num-w2.num-3)%5 == 0
}

// func (w WuXin) String() string {
// 	return w.name
// }
