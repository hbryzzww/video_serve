package controllers

import (
	"github.com/astaxie/beego"
	"liamxin/video_serve/models"
)

type VideoController struct {
	beego.Controller
}

// @router / [get]
func (controller *VideoController) List() {
	controller.Data["json"] = "{name:Tom}"
	controller.ServeJSON()
}

// @router /get [get]
func (c *VideoController) Get() {
	v, e := models.GetVideo()

	if e != nil {
		c.Data["json"] = "error"
	} else {
		c.Data["json"] = v
	}
	c.ServeJSON()
}
