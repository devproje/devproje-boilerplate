package sample

type SampleService struct{}

func (s *SampleService) Name() string {
	return "sample"
}

func (s *SampleService) Init() error {
	return nil
}

func (s *SampleService) Destroy() error {
	return nil
}

func (*SampleService) Health() bool {
	return true
}

var SampleServiceModule = &SampleService{}
