package models

import (
	"errors"
	"fmt"
	"github.com/astaxie/beego/orm"
)

type Video struct {
	Id    int64
	Name  string
	Uri   string
	Cover string
}

func init() {
	orm.RegisterModel(new(Video))
}

func GetVideo() (u *Video, err error) {
	o := orm.NewOrm()
	video := Video{Id: 11}

	if ok := o.Read(&video); ok == orm.ErrNoRows {
		return nil, errors.New("查询不到")
	} else if ok == orm.ErrMissPK {
		return nil, errors.New("找不到主键")
	} else {
		return &video, nil
	}
}

func findById(id int64) Video {
	o := orm.NewOrm()
	user := Video{Id: id}

	err := o.Read(&user)

	if err == orm.ErrNoRows {
		fmt.Println("查询不到")
	} else if err == orm.ErrMissPK {
		fmt.Println("找不到主键")
	} else {
		fmt.Println(user.Id, user.Name)
	}
	return user
}
