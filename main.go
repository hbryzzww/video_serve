package main

import (
	"github.com/astaxie/beego/orm"
	//"liamxin/video_serve/models"
	_ "github.com/go-sql-driver/mysql"
	_ "liamxin/video_serve/routers"

	"github.com/astaxie/beego"
)

func init() {
	orm.RegisterDataBase("default", beego.AppConfig.String("dbtype"), beego.AppConfig.String("sqlconn"))
	//调用 RunCommand 执行 orm 命令。
	orm.RunCommand()

}

func main() {
	if beego.BConfig.RunMode == "dev" {
		beego.BConfig.WebConfig.DirectoryIndex = true
		beego.BConfig.WebConfig.StaticDir["/swagger"] = "swagger"
	}
	beego.Run()
	//orm.RegisterDataBase("default", "mysql", "root:root123@tcp(127.0.0.1:3306)/dev_video?charset=utf8")
}
