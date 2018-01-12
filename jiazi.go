package ganzhiwuxin

type JiaZi struct {
	t TianGan
	d DiZhi
}

func (jz *JiaZi) Init(t TianGan, d DiZhi) bool {
	if (t.Is阳() && d.Is阳()) || (!t.Is阳() && !d.Is阳()) {
		jz.t = t
		jz.d = d
		return true
	}
	return false
}

func (jz JiaZi) GetGan() TianGan {
	return jz.t
}

func (jz JiaZi) GetDiZhi() DiZhi {
	return jz.d
}

func (jz JiaZi) Eq(h JiaZi) bool {
	return jz.t.Eq(h.t) && jz.d.Eq(h.d)
}

func (jz JiaZi) GetNum() int {
	var num int
	var 甲 TianGan
	var 子 DiZhi
	甲.Init("甲")
	子.Init("子")
	for i := 0; i < 60; i++ {
		var a JiaZi
		a.Init(甲.Add(i), 子.Add(i))
		if a.Eq(jz) {
			num = i + 1
			break
		}
	}
	return num
}
