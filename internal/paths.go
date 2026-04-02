package internal

import (
	"os"
	"path/filepath"
)

func BaseDir() string {
	home, _ := os.UserHomeDir()
	return filepath.Join(home, ".rubbish")
}

func FilesDir() string {
	return filepath.Join(BaseDir(), "files")
}

func MetadataPath() string {
	return filepath.Join(BaseDir(), "metadata.json")
}

func EnsureDirs() error {
	return os.MkdirAll(FilesDir(), 0755)
}
