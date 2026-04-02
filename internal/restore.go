package internal

import (
	"os"
	"path/filepath"
)

func Restore(id string) error {
	entries, err := LoadMetadata()
	if err != nil {
		return err
	}

	var updated []Entry

	for _, e := range entries {
		if e.ID == id {
			src := filepath.Join(FilesDir(), e.StoredName)

			err := os.Rename(src, e.OriginalPath)
			if err != nil {
				return err
			}
			continue
		}
		updated = append(updated, e)
	}

	return SaveMetadata(updated)
}
