package modules

import (
	"log"
	"slices"
)

type ServiceModule interface {
	Name() string
	Init() error
	Destroy() error
}

type ModuleLoader struct {
	modules []ServiceModule
}

func (sl *ModuleLoader) Insmod(module ServiceModule) {
	sl.modules = append(sl.modules, module)
}

func (sl *ModuleLoader) Load() {
	var err error

	for _, m := range sl.modules {
		err = m.Init()
		if err != nil {
			log.Printf("Failed to load module: %s", m.Name())
			continue
		}

		log.Printf("Loaded module: %s", m.Name())
	}
}

func (sl *ModuleLoader) Unload() {
	var err error
	slices.Reverse(sl.modules)

	for _, m := range sl.modules {
		err = m.Destroy()
		if err != nil {
			log.Printf("Failed to unload module: %s", m.Name())
			continue
		}

		log.Printf("Unloaded module: %s", m.Name())
	}
}

var LOADER = &ModuleLoader{
	modules: []ServiceModule{},
}
