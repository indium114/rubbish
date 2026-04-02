package internal

import (
	"os"
	"path/filepath"
)

func Delete(id string) error {
	entries, err := LoadMetadata()
	if err != nil {
		return err
	}

	var updated []Entry

	for _, e := range entries {
		if e.ID == id {
			path := filepath.Join(FilesDir(), e.StoredName)
			os.RemoveAll(path)
			continue
		}
		updated = append(updated, e)
	}

	return SaveMetadata(updated)
}

func Clear() error {
	err := os.RemoveAll(FilesDir())
	if err != nil {
		return err
	}

	err = EnsureDirs()
	if err != nil {
		return err
	}

	return SaveMetadata([]Entry{})
}
