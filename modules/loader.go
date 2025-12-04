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
		log.Printf("[%s] Loading module...\n", m.Name())
		err = m.Init()
		if err != nil {
			log.Printf("[%s] Failed to load module\n", m.Name())
			log.Printf("[%s] %v\n", m.Name(), err)
			continue
		}

		log.Printf("[%s] Loaded module\n", m.Name())
	}
}

func (sl *ModuleLoader) Unload() {
	var err error
	slices.Reverse(sl.modules)

	for _, m := range sl.modules {
		log.Printf("[%s] Unloading module...\n", m.Name())
		err = m.Destroy()
		if err != nil {
			log.Printf("[%s] Failed to unload module\n", m.Name())
			continue
		}

		log.Printf("[%s] Unloaded module\n", m.Name())
	}
}

var LOADER = &ModuleLoader{
	modules: []ServiceModule{},
}
