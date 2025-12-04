package routes

import (
	"git.wh64.net/devproje/devproje-boilerplate/modules/sample"
	"github.com/gin-gonic/gin"
)

func health(ctx *gin.Context) {
	var ok = sample.SampleServiceModule.Health()
	if !ok {
		ctx.JSON(500, gin.H{"ok": 0, "errno": "Your webserver health is not online"})
		return
	}

	ctx.JSON(200, gin.H{"ok": 1, "status": "Your webserver health is ok"})
}

func API(app *gin.Engine) {
	v1 := app.Group("/v1")
	v1.GET("/health", health)
}
